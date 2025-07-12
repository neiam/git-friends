use clap::{Command, Arg};
use git_friends::{Config, Result, auth::AuthManager, mqtt::MqttClient, git::CommitInfo};
use log::{info, error, warn};
use std::sync::Arc;
use warp::{Filter, Reply};
use warp::http::StatusCode;

#[derive(Clone)]
struct ServerState {
    mqtt_client: Arc<MqttClient>,
    auth_manager: Arc<AuthManager>,
    config: Arc<Config>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let matches = Command::new("gf-server")
        .version("0.1.0")
        .about("Git Friends HTTP server - receives commit webhooks and publishes to MQTT")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
        )
        .arg(
            Arg::new("bind")
                .short('b')
                .long("bind")
                .value_name("ADDRESS")
                .help("Bind address (e.g., 0.0.0.0:8080)")
        )
        .arg(
            Arg::new("generate-token")
                .long("generate-token")
                .value_name("USERNAME")
                .help("Generate a new authentication token for the specified username")
        )
        .get_matches();
    
    if let Some(username) = matches.get_one::<String>("generate-token") {
        let token_config = AuthManager::generate_token_config(username);
        println!("Generated token for user '{}': {}", token_config.username, token_config.token);
        println!("Add this to your configuration file:");
        println!("[[auth.tokens]]");
        println!("token = \"{}\"", token_config.token);
        println!("username = \"{}\"", token_config.username);
        return Ok(());
    }
    
    info!("Starting gf-server");
    
    // Load configuration
    let mut config = match matches.get_one::<String>("config") {
        Some(path) => {
            std::env::set_var("GIT_FRIENDS_CONFIG", path);
            Config::new()?
        }
        None => Config::new()?
    };
    
    // Modify client_id to append "/server"
    config.mqtt.client_id = format!("{}/server", config.mqtt.client_id);
    
    // Override bind address if provided
    let bind_address = matches.get_one::<String>("bind")
        .map(|s| s.as_str())
        .unwrap_or(&config.server.bind_address)
        .to_string();
    
    info!("Configuration loaded");
    info!("MQTT broker: {}:{}", config.mqtt.broker_host, config.mqtt.broker_port);
    info!("Server binding to: {}", bind_address);
    
    // Create MQTT client
    let (mqtt_client, mut mqtt_events) = MqttClient::new(config.mqtt.clone())?;
    let mqtt_client = Arc::new(mqtt_client);
    
    // Create auth manager
    let auth_manager = Arc::new(AuthManager::new(&config.auth));
    
    // Create server state
    let state = ServerState {
        mqtt_client: mqtt_client.clone(),
        auth_manager: auth_manager.clone(),
        config: Arc::new(config),
    };
    
    // Wait for MQTT connection
    info!("Connecting to MQTT broker...");
    mqtt_client.wait_for_connection().await?;
    info!("Connected to MQTT broker");
    
    // Spawn task to handle MQTT events
    tokio::spawn(async move {
        while let Some(event) = mqtt_events.recv().await {
            match event {
                rumqttc::Event::Incoming(rumqttc::Packet::ConnAck(_)) => {
                    info!("MQTT connection acknowledged");
                }
                rumqttc::Event::Incoming(rumqttc::Packet::Disconnect) => {
                    warn!("MQTT disconnected");
                }
                _ => {}
            }
        }
    });
    
    // Define routes
    let webhook_route = warp::path("webhook")
        .and(warp::post())
        .and(warp::header::optional::<String>("authorization"))
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handle_webhook);
    
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::with_status("OK", StatusCode::OK));
    
    let routes = webhook_route
        .or(health_route)
        .with(warp::log("gf-server"));
    
    // Parse bind address
    let addr: std::net::SocketAddr = bind_address.parse()
        .map_err(|e| git_friends::GitFriendsError::InvalidConfig(format!("Invalid bind address: {}", e)))?;
    
    info!("Server starting on {}", addr);
    
    // Start server
    warp::serve(routes)
        .run(addr)
        .await;
    
    Ok(())
}

fn with_state(state: ServerState) -> impl Filter<Extract = (ServerState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn handle_webhook(
    auth_header: Option<String>,
    commit_info: CommitInfo,
    state: ServerState,
) -> std::result::Result<impl Reply, warp::Rejection> {
    // Validate authentication and get username
    let username = if let Some(header) = auth_header {
        match state.auth_manager.validate_bearer_token(&header) {
            Ok(Some(username)) => {
                info!("Authentication successful for user: {}", username);
                Some(username)
            }
            Ok(None) => {
                warn!("Authentication failed - invalid token");
                return Ok(warp::reply::with_status(
                    "Authentication failed",
                    StatusCode::UNAUTHORIZED,
                ));
            }
            Err(e) => {
                warn!("Authentication error: {}", e);
                return Ok(warp::reply::with_status(
                    "Authentication failed",
                    StatusCode::UNAUTHORIZED,
                ));
            }
        }
    } else if state.config.auth.require_auth {
        warn!("Authentication required but not provided");
        return Ok(warp::reply::with_status(
            "Authentication required",
            StatusCode::UNAUTHORIZED,
        ));
    } else {
        Some("anonymous".to_string())
    };
    
    info!("Processing commit: {} by {} - {}", 
          commit_info.short_hash, 
          commit_info.author_name, 
          commit_info.message.lines().next().unwrap_or(""));
    
    // Publish to MQTT with username
    match state.mqtt_client.publish_commit(&commit_info, username.as_deref()).await {
        Ok(()) => {
            info!("Published commit to MQTT");
            Ok(warp::reply::with_status(
                "Commit processed successfully",
                StatusCode::OK,
            ))
        }
        Err(e) => {
            error!("Failed to publish commit to MQTT: {}", e);
            Ok(warp::reply::with_status(
                "Failed to process commit",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

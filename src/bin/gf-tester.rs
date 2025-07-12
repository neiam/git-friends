use clap::{Command, Arg};
use git_friends::{Config, Result, mqtt::MqttClient, git::CommitInfo};
use log::{info, error};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let matches = Command::new("gf-tester")
        .version("0.1.0")
        .about("Git Friends Test Client - sends mock commit messages to test the system")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
        )
        .arg(
            Arg::new("count")
                .short('n')
                .long("count")
                .value_name("NUMBER")
                .help("Number of test commits to send")
                .default_value("5")
        )
        .arg(
            Arg::new("interval")
                .short('i')
                .long("interval")
                .value_name("SECONDS")
                .help("Interval between commits in seconds")
                .default_value("2")
        )
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .value_name("USERNAME")
                .help("Username for authentication (if required)")
        )
        .arg(
            Arg::new("repo")
                .short('r')
                .long("repo")
                .value_name("REPO_URL")
                .help("Mock repository URL")
                .default_value("https://github.com/test/mock-repo")
        )
        .arg(
            Arg::new("author")
                .short('a')
                .long("author")
                .value_name("AUTHOR")
                .help("Mock author name")
                .default_value("Test Author")
        )
        .arg(
            Arg::new("branch")
                .short('b')
                .long("branch")
                .value_name("BRANCH")
                .help("Mock branch name")
                .default_value("master")
        )
        .arg(
            Arg::new("continuous")
                .long("continuous")
                .help("Send commits continuously until interrupted")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();
    
    info!("Starting gf-tester");
    
    // Load configuration
    let mut config = match matches.get_one::<String>("config") {
        Some(path) => {
            std::env::set_var("GIT_FRIENDS_CONFIG", path);
            Config::new()?
        }
        None => Config::new()?
    };
    
    // Modify client_id to append "/tester"
    config.mqtt.client_id = format!("{}/tester", config.mqtt.client_id);
    
    let count: u32 = matches.get_one::<String>("count")
        .unwrap()
        .parse()
        .map_err(|_| git_friends::GitFriendsError::InvalidConfig("Invalid count value".to_string()))?;
    
    let interval: u64 = matches.get_one::<String>("interval")
        .unwrap()
        .parse()
        .map_err(|_| git_friends::GitFriendsError::InvalidConfig("Invalid interval value".to_string()))?;
    
    let username = matches.get_one::<String>("username").map(|s| s.as_str());
    let repo_url = matches.get_one::<String>("repo").unwrap();
    let author = matches.get_one::<String>("author").unwrap();
    let branch = matches.get_one::<String>("branch").unwrap();
    let continuous = matches.get_flag("continuous");
    
    info!("Configuration loaded");
    info!("MQTT broker: {}:{}", config.mqtt.broker_host, config.mqtt.broker_port);
    info!("MQTT client ID: {}", config.mqtt.client_id);
    info!("Test parameters:");
    info!("  Repository: {}", repo_url);
    info!("  Author: {}", author);
    info!("  Branch: {}", branch);
    info!("  Count: {}", if continuous { "continuous".to_string() } else { count.to_string() });
    info!("  Interval: {} seconds", interval);
    
    // Create MQTT client
    let (mqtt_client, mut mqtt_events) = MqttClient::new(config.mqtt.clone())?;
    
    // Connect to MQTT broker
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
                    error!("MQTT disconnected");
                }
                rumqttc::Event::Incoming(rumqttc::Packet::PubAck(_)) => {
                    info!("Publish acknowledged");
                }
                _ => {}
            }
        }
    });
    
    // Generate and send mock commits
    let mut sent_count = 0;
    let mut iteration = 0;
    
    loop {
        iteration += 1;
        
        let commit_info = generate_mock_commit(
            repo_url,
            author,
            branch,
            iteration,
        );
        
        info!("Sending test commit {}: {} - {}", 
              iteration, 
              commit_info.short_hash, 
              commit_info.message.lines().next().unwrap_or(""));
        
        match mqtt_client.publish_commit(&commit_info, username).await {
            Ok(()) => {
                info!("Successfully published commit {}", commit_info.short_hash);
                sent_count += 1;
            }
            Err(e) => {
                error!("Failed to publish commit: {}", e);
            }
        }
        
        if !continuous && sent_count >= count {
            break;
        }
        
        if continuous {
            info!("Sleeping for {} seconds... (Press Ctrl+C to stop)", interval);
        } else if sent_count < count {
            info!("Sleeping for {} seconds...", interval);
        }
        
        sleep(Duration::from_secs(interval)).await;
    }
    
    info!("Sent {} test commits", sent_count);
    
    // Give a moment for the last message to be sent
    sleep(Duration::from_secs(1)).await;
    
    Ok(())
}

fn generate_mock_commit(repo_url: &str, author: &str, branch: &str, iteration: u32) -> CommitInfo {
    let commit_id = Uuid::new_v4().to_string().replace("-", "");
    let short_hash = format!("{:.7}", commit_id);
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    let messages = vec![
        "Add new feature for user authentication",
        "Fix bug in payment processing",
        "Update dependencies to latest versions",
        "Refactor database connection handling",
        "Add unit tests for core functionality",
        "Improve error handling in API endpoints",
        "Update documentation with new examples",
        "Optimize database queries for performance",
        "Add logging for better debugging",
        "Fix memory leak in background tasks",
        "Implement rate limiting for API",
        "Add support for multiple languages",
        "Fix CSS styling issues on mobile",
        "Add validation for user input",
        "Update CI/CD pipeline configuration",
        "Fix timezone handling in date calculations",
        "Add metrics collection for monitoring",
        "Implement caching for improved performance",
        "Fix race condition in concurrent operations",
        "Add backup and recovery functionality",
    ];
    
    let files_sets = vec![
        vec!["src/main.rs".to_string(), "Cargo.toml".to_string()],
        vec!["src/auth.rs".to_string(), "src/lib.rs".to_string(), "tests/auth_test.rs".to_string()],
        vec!["src/api/mod.rs".to_string(), "src/api/handlers.rs".to_string()],
        vec!["src/database.rs".to_string(), "migrations/001_initial.sql".to_string()],
        vec!["README.md".to_string(), "docs/api.md".to_string()],
        vec!["src/utils.rs".to_string(), "src/config.rs".to_string(), "src/errors.rs".to_string()],
        vec!["frontend/src/App.js".to_string(), "frontend/src/components/Login.js".to_string()],
        vec!["docker-compose.yml".to_string(), "Dockerfile".to_string()],
        vec!["src/models/user.rs".to_string(), "src/models/mod.rs".to_string()],
        vec!["tests/integration_test.rs".to_string()],
    ];
    
    let message_index = (iteration as usize - 1) % messages.len();
    let files_index = (iteration as usize - 1) % files_sets.len();
    
    let author_email = format!("{}@example.com", author.to_lowercase().replace(" ", "."));
    
    CommitInfo {
        hash: commit_id.clone(),
        short_hash: short_hash.clone(),
        author_name: author.to_string(),
        author_email: author_email.clone(),
        committer_name: author.to_string(),
        committer_email: author_email,
        message: messages[message_index].to_string(),
        timestamp,
        repository_url: repo_url.to_string(),
        branch: branch.to_string(),
        files_changed: files_sets[files_index].clone(),
    }
}


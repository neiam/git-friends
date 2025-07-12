use clap::{Command, Arg};
use git_friends::{Config, Result, mqtt::{MqttClient, extract_commit_from_mqtt_message, format_commit_for_irc}};
use log::{info, error, warn};
use irc::client::prelude::*;
use futures_util::stream::StreamExt;
use rumqttc::{Event, Packet};
use irc::proto::Command as IrcCommand;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let matches = Command::new("gf-irc")
        .version("0.1.0")
        .about("Git Friends IRC client - listens to MQTT topics and publishes to IRC")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
        )
        .arg(
            Arg::new("nick")
                .short('n')
                .long("nick")
                .value_name("NICKNAME")
                .help("IRC nickname")
        )
        .arg(
            Arg::new("server")
                .short('s')
                .long("server")
                .value_name("SERVER")
                .help("IRC server")
        )
        .arg(
            Arg::new("channels")
                .long("channels")
                .value_name("CHANNELS")
                .help("Comma-separated list of IRC channels")
        )
        .arg(
            Arg::new("mqtt-topics")
                .long("mqtt-topics")
                .value_name("TOPICS")
                .help("Comma-separated list of MQTT topics to subscribe to")
        )
        .get_matches();
    
    info!("Starting gf-irc");
    
    // Load configuration
    let mut config = match matches.get_one::<String>("config") {
        Some(path) => {
            std::env::set_var("GIT_FRIENDS_CONFIG", path);
            Config::new()?
        }
        None => Config::new()?
    };
    
    // Modify client_id to append "/irc"
    config.mqtt.client_id = format!("{}/irc", config.mqtt.client_id);
    
    // Override configuration with command line arguments
    if let Some(nick) = matches.get_one::<String>("nick") {
        config.irc.nick = nick.clone();
        config.irc.username = nick.clone();
    }
    
    if let Some(server) = matches.get_one::<String>("server") {
        config.irc.server = server.clone();
    }
    
    if let Some(channels) = matches.get_one::<String>("channels") {
        config.irc.channels = channels.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    if let Some(topics) = matches.get_one::<String>("mqtt-topics") {
        config.irc.topic_filters = topics.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    info!("Configuration loaded");
    info!("IRC server: {}:{}", config.irc.server, config.irc.port);
    info!("IRC nick: {}", config.irc.nick);
    info!("IRC channels: {:?}", config.irc.channels);
    info!("MQTT topics: {:?}", config.irc.topic_filters);
    
    // Create IRC client
    let irc_config = irc::client::data::Config {
        nickname: Some(config.irc.nick.clone()),
        username: Some(config.irc.username.clone()),
        realname: Some(config.irc.real_name.clone()),
        server: Some(config.irc.server.clone()),
        port: Some(config.irc.port),
        use_tls: Some(config.irc.use_tls),
        channels: config.irc.channels.clone(),
        ..Default::default()
    };
    
    let mut irc_client = Client::from_config(irc_config).await?;
    
    // Create MQTT client
    let mqtt_config = config.mqtt.clone();
    let (mqtt_client, mut mqtt_events) = MqttClient::new(mqtt_config)?;
    
    // Connect to IRC
    info!("Connecting to IRC...");
    irc_client.identify()?;
    info!("Connected to IRC");
    
    // Subscribe to MQTT topics
    info!("Connecting to MQTT broker...");
    mqtt_client.wait_for_connection().await?;
    mqtt_client.subscribe_to_topics(&config.irc.topic_filters).await?;
    info!("Subscribed to MQTT topics");
    
    // Clone data for the IRC message handler
    let irc_channels = config.irc.channels.clone();
    let irc_client_sender = irc_client.sender();
    
    // Spawn MQTT message handler
    tokio::spawn(async move {
        while let Some(event) = mqtt_events.recv().await {
            match event {
                Event::Incoming(Packet::Publish(publish)) => {
                    match extract_commit_from_mqtt_message(&publish.payload) {
                        Ok(commit_info) => {
                            let message = format_commit_for_irc(&commit_info);
                            info!("Received commit: {}", message);
                            
                            // Send to all configured channels
                            for channel in &irc_channels {
                                if let Err(e) = irc_client_sender.send_privmsg(channel, &message) {
                                    error!("Failed to send message to IRC channel {}: {}", channel, e);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse MQTT message: {}", e);
                        }
                    }
                }
                Event::Incoming(Packet::ConnAck(_)) => {
                    info!("MQTT connection acknowledged");
                }
                Event::Incoming(Packet::Disconnect) => {
                    warn!("MQTT disconnected");
                }
                _ => {}
            }
        }
    });
    
    // Handle IRC messages
    let mut irc_stream = irc_client.stream()?;
    
    while let Some(message) = irc_stream.next().await.transpose()? {
        match message.command {
            IrcCommand::PRIVMSG(ref target, ref msg) => {
                if msg.starts_with("!git-friends") {
                    let response = "Git Friends IRC bot - monitoring git commits via MQTT";
                    irc_client.send_privmsg(target, response)?;
                }
            }
            IrcCommand::Response(Response::RPL_WELCOME, _) => {
                info!("Successfully connected to IRC server");
            }
            IrcCommand::Response(Response::RPL_ENDOFMOTD, _) => {
                info!("End of MOTD received");
            }
            _ => {}
        }
    }
    
    Ok(())
}

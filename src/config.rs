use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use config::{Config as ConfigImpl, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub mqtt: MqttConfig,
    pub irc: IrcConfig,
    pub git: GitConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub bind_address: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MqttConfig {
    pub broker_host: String,
    pub broker_port: u16,
    pub client_id: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub topic_prefix: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IrcConfig {
    pub server: String,
    pub port: u16,
    pub nick: String,
    pub username: String,
    pub real_name: String,
    pub channels: Vec<String>,
    pub use_tls: bool,
    pub topic_filters: Vec<String>, // MQTT topics to listen to
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitConfig {
    pub hook_script_path: Option<String>,
    pub repository_mappings: HashMap<String, String>, // repo_url -> topic_suffix
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    pub tokens: Vec<TokenConfig>,
    pub require_auth: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenConfig {
    pub token: String,
    pub username: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                host: "localhost".to_string(),
                port: 8080,
                bind_address: "0.0.0.0:8080".to_string(),
            },
            mqtt: MqttConfig {
                broker_host: "localhost".to_string(),
                broker_port: 1883,
                client_id: "git-friends".to_string(),
                username: None,
                password: None,
                topic_prefix: "git-friends".to_string(),
            },
            irc: IrcConfig {
                server: "irc.libera.chat".to_string(),
                port: 6667,
                nick: "git-friends".to_string(),
                username: "git-friends".to_string(),
                real_name: "Git Friends Bot".to_string(),
                channels: vec!["#git-friends".to_string()],
                use_tls: false,
                topic_filters: vec!["git-friends/+/+".to_string()],
            },
            git: GitConfig {
                hook_script_path: None,
                repository_mappings: HashMap::new(),
            },
            auth: AuthConfig {
                tokens: vec![],
                require_auth: true,
            },
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = ConfigImpl::builder()
            .set_default("server.host", "localhost")?
            .set_default("server.port", 8080)?
            .set_default("server.bind_address", "0.0.0.0:8080")?
            .set_default("mqtt.broker_host", "localhost")?
            .set_default("mqtt.broker_port", 1883)?
            .set_default("mqtt.client_id", "git-friends")?
            .set_default("mqtt.topic_prefix", "git-friends")?
            .set_default("irc.server", "irc.libera.chat")?
            .set_default("irc.port", 6667)?
            .set_default("irc.nick", "git-friends")?
            .set_default("irc.username", "git-friends")?
            .set_default("irc.real_name", "Git Friends Bot")?
            .set_default("irc.use_tls", false)?
            .set_default("auth.require_auth", true)?;
        
        // Try to load from config file
        if let Ok(config_path) = env::var("GIT_FRIENDS_CONFIG") {
            config = config.add_source(File::with_name(&config_path).required(false));
        } else {
            // Try common config locations
            config = config
                .add_source(File::with_name("git-friends.toml").required(false))
                .add_source(File::with_name("config/git-friends.toml").required(false))
                .add_source(File::with_name("/etc/git-friends.toml").required(false));
        }
        
        // Override with environment variables
        config = config.add_source(Environment::with_prefix("GIT_FRIENDS"));
        
        config.build()?.try_deserialize()
    }
    
    pub fn mqtt_topic_for_repo(&self, repo_url: &str, committer: &str, username: Option<&str>) -> String {
        let repo_suffix = self.git.repository_mappings
            .get(repo_url)
            .cloned()
            .unwrap_or_else(|| Self::sanitize_repo_url(repo_url));
        
        if let Some(username) = username {
            format!("{}/{}/{}/{}", self.mqtt.topic_prefix, username, repo_suffix, committer)
        } else {
            format!("{}/{}/{}", self.mqtt.topic_prefix, repo_suffix, committer)
        }
    }
    
    fn sanitize_repo_url(url: &str) -> String {
        url.replace("https://", "")
           .replace("http://", "")
           .replace("git@", "")
           .replace(":", "/")
           .replace(".git", "")
           .replace("/", "_")
           .replace(".", "_")
    }
}

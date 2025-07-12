use thiserror::Error;

pub type Result<T> = std::result::Result<T, GitFriendsError>;

#[derive(Error, Debug)]
pub enum GitFriendsError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("MQTT error: {0}")]
    Mqtt(#[from] rumqttc::ClientError),

    #[error("IRC error: {0}")]
    Irc(#[from] irc::error::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] warp::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

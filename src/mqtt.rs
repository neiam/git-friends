use crate::config::MqttConfig;
use crate::errors::Result;
use crate::git::CommitInfo;
use log::{error, info};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

pub struct MqttClient {
    client: AsyncClient,
    config: MqttConfig,
}

impl MqttClient {
    pub fn new(config: MqttConfig) -> Result<(Self, mpsc::Receiver<rumqttc::Event>)> {
        let mut mqtt_options =
            MqttOptions::new(&config.client_id, &config.broker_host, config.broker_port);

        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            mqtt_options.set_credentials(username, password);
        }

        mqtt_options.set_keep_alive(Duration::from_secs(30));
        mqtt_options.set_max_packet_size(1024 * 1024, 1024 * 1024); // 1MB

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

        // Create a channel for events
        let (tx, rx) = mpsc::channel(100);

        // Spawn event loop
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        if tx.send(event).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        error!("MQTT connection error: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });

        Ok((MqttClient { client, config }, rx))
    }

    pub async fn publish_commit(
        &self,
        commit_info: &CommitInfo,
        username: Option<&str>,
    ) -> Result<()> {
        let topic = if let Some(username) = username {
            format!(
                "{}/{}/{}/{}",
                self.config.topic_prefix,
                username,
                commit_info
                    .repository_url
                    .replace("https://", "")
                    .replace("http://", "")
                    .replace("/", "_"),
                commit_info.committer_name
            )
        } else {
            format!(
                "{}/{}/{}",
                self.config.topic_prefix,
                commit_info
                    .repository_url
                    .replace("https://", "")
                    .replace("http://", "")
                    .replace("/", "_"),
                commit_info.committer_name
            )
        };

        let payload = serde_json::to_string(commit_info)?;

        info!("Publishing commit to topic: {}", topic);

        self.client
            .publish(&topic, QoS::AtMostOnce, false, payload)
            .await?;

        Ok(())
    }

    pub async fn subscribe_to_topics(&self, topics: &[String]) -> Result<()> {
        for topic in topics {
            info!("Subscribing to topic: {}", topic);
            self.client.subscribe(topic, QoS::AtMostOnce).await?;
        }
        Ok(())
    }

    pub async fn wait_for_connection(&self) -> Result<()> {
        // Wait for connection to be established
        timeout(Duration::from_secs(10), async {
            loop {
                if self
                    .client
                    .try_publish("$SYS/test", QoS::AtMostOnce, false, "test")
                    .is_ok()
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .map_err(|_| {
            crate::errors::GitFriendsError::ConnectionFailed("MQTT connection timeout".to_string())
        })?;

        Ok(())
    }
}

pub fn extract_commit_from_mqtt_message(payload: &[u8]) -> Result<CommitInfo> {
    let payload_str = String::from_utf8_lossy(payload);
    let commit_info: CommitInfo = serde_json::from_str(&payload_str)?;
    Ok(commit_info)
}

pub fn format_commit_for_irc(commit_info: &CommitInfo) -> String {
    format!(
        "[{}] {} by {} ({}): {} - {}",
        commit_info.short_hash,
        commit_info
            .repository_url
            .split('/')
            .next_back()
            .unwrap_or("unknown"),
        commit_info.author_name,
        commit_info.branch,
        commit_info.message.lines().next().unwrap_or("").trim(),
        if commit_info.files_changed.len() > 3 {
            format!("{} files changed", commit_info.files_changed.len())
        } else {
            commit_info.files_changed.join(", ")
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MqttConfig;

    #[test]
    fn test_commit_formatting() {
        let commit_info = CommitInfo {
            hash: "abcdef1234567890".to_string(),
            short_hash: "abcdef1".to_string(),
            author_name: "John Doe".to_string(),
            author_email: "john@example.com".to_string(),
            committer_name: "John Doe".to_string(),
            committer_email: "john@example.com".to_string(),
            message: "Fix bug in authentication\n\nThis fixes the issue where...".to_string(),
            timestamp: 1234567890,
            repository_url: "https://github.com/user/repo".to_string(),
            branch: "main".to_string(),
            files_changed: vec!["src/auth.rs".to_string(), "tests/auth_test.rs".to_string()],
        };

        let formatted = format_commit_for_irc(&commit_info);
        assert!(formatted.contains("abcdef1"));
        assert!(formatted.contains("John Doe"));
        assert!(formatted.contains("Fix bug in authentication"));
    }
}

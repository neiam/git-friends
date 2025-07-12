use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;
use crate::config::{AuthConfig, TokenConfig};
use crate::errors::{Result, GitFriendsError};

pub struct AuthManager {
    tokens: HashMap<String, String>, // token -> username
    require_auth: bool,
}

impl AuthManager {
    pub fn new(config: &AuthConfig) -> Self {
        let tokens = config.tokens.iter()
            .map(|tc| (tc.token.clone(), tc.username.clone()))
            .collect();
        
        AuthManager {
            tokens,
            require_auth: config.require_auth,
        }
    }
    
    pub fn validate_token(&self, token: &str) -> Option<&str> {
        if !self.require_auth {
            return Some("anonymous");
        }
        
        self.tokens.get(token).map(|s| s.as_str())
    }
    
    pub fn validate_bearer_token(&self, auth_header: &str) -> Result<Option<String>> {
        if !self.require_auth {
            return Ok(Some("anonymous".to_string()));
        }
        
        if !auth_header.starts_with("Bearer ") {
            return Err(GitFriendsError::AuthenticationFailed);
        }
        
        let token = &auth_header[7..]; // Remove "Bearer " prefix
        Ok(self.tokens.get(token).cloned())
    }
    
    pub fn generate_token() -> String {
        let uuid = Uuid::new_v4();
        general_purpose::URL_SAFE_NO_PAD.encode(uuid.as_bytes())
    }
    
    pub fn generate_token_config(username: &str) -> TokenConfig {
        TokenConfig {
            token: Self::generate_token(),
            username: username.to_string(),
        }
    }
    
    pub fn add_token(&mut self, token: String, username: String) {
        self.tokens.insert(token, username);
    }
    
    pub fn remove_token(&mut self, token: &str) -> Option<String> {
        self.tokens.remove(token)
    }
    
    pub fn list_tokens(&self) -> Vec<TokenConfig> {
        self.tokens.iter()
            .map(|(token, username)| TokenConfig {
                token: token.clone(),
                username: username.clone(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AuthConfig;
    
    #[test]
    fn test_auth_manager_with_tokens() {
        let config = AuthConfig {
            tokens: vec![TokenConfig {
                token: "test-token".to_string(),
                username: "testuser".to_string(),
            }],
            require_auth: true,
        };
        
        let auth_manager = AuthManager::new(&config);
        
        assert_eq!(auth_manager.validate_token("test-token"), Some("testuser"));
        assert_eq!(auth_manager.validate_token("invalid-token"), None);
    }
    
    #[test]
    fn test_auth_manager_without_auth() {
        let config = AuthConfig {
            tokens: vec![],
            require_auth: false,
        };
        
        let auth_manager = AuthManager::new(&config);
        
        assert_eq!(auth_manager.validate_token("any-token"), Some("anonymous"));
        assert_eq!(auth_manager.validate_token(""), Some("anonymous"));
    }
    
    #[test]
    fn test_bearer_token_validation() {
        let config = AuthConfig {
            tokens: vec![TokenConfig {
                token: "test-token".to_string(),
                username: "testuser".to_string(),
            }],
            require_auth: true,
        };
        
        let auth_manager = AuthManager::new(&config);
        
        assert_eq!(auth_manager.validate_bearer_token("Bearer test-token").unwrap(), Some("testuser".to_string()));
        assert_eq!(auth_manager.validate_bearer_token("Bearer invalid-token").unwrap(), None);
        assert!(auth_manager.validate_bearer_token("Invalid format").is_err());
    }
    
    #[test]
    fn test_token_generation() {
        let token1 = AuthManager::generate_token();
        let token2 = AuthManager::generate_token();
        
        assert_ne!(token1, token2);
        assert!(!token1.is_empty());
        assert!(!token2.is_empty());
    }
    
    #[test]
    fn test_token_config_generation() {
        let config = AuthManager::generate_token_config("testuser");
        
        assert_eq!(config.username, "testuser");
        assert!(!config.token.is_empty());
    }
}

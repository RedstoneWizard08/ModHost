//! ModHost's auth configuration.

use modhost_core::Result;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};

/// ModHost's auth methods configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfigs {
    /// The config for GitHub's OAuth2 system.
    pub github: AuthConfig,
}

/// The configuration for an OAuth2 configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    /// The OAuth2 client ID.
    pub client_id: String,

    /// The OAuth2 client secret.
    pub client_secret: String,
}

impl AuthConfigs {
    /// Get the OAuth2 client for GitHub.
    pub fn github(&self) -> Result<BasicClient> {
        Ok(BasicClient::new(
            ClientId::new(self.github.client_id.clone()),
            Some(ClientSecret::new(self.github.client_secret.clone())),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?,
            Some(TokenUrl::new(
                "https://github.com/login/oauth/access_token".to_string(),
            )?),
        ))
    }
}

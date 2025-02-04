//! ModHost's auth configuration.

use modhost_core::Result;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, TokenUrl,
};

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
    pub fn github(
        &self,
    ) -> Result<BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>>
    {
        Ok(
            BasicClient::new(ClientId::new(self.github.client_id.clone()))
                .set_client_secret(ClientSecret::new(self.github.client_secret.clone()))
                .set_auth_uri(AuthUrl::new(
                    "https://github.com/login/oauth/authorize".to_string(),
                )?)
                .set_token_uri(TokenUrl::new(
                    "https://github.com/login/oauth/access_token".to_string(),
                )?),
        )
    }
}

use codex_app_server_protocol::AuthMode;
use serde::{Deserialize, Serialize};

/// Mode for storing CLI authentication credentials.
/// Note: This is kept for backward compatibility but is not actively used
/// since authentication is now read directly from config files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthCredentialsStoreMode {
    /// Store credentials in a file in the FreeCode home directory.
    File,
    /// Store credentials in an OS-specific keyring service.
    Keyring,
    /// Automatically choose between keyring and file based on availability.
    Auto,
}

impl Default for AuthCredentialsStoreMode {
    fn default() -> Self {
        Self::File
    }
}

/// Simple API Key based authentication.
/// Authentication is read from ~/.freecode/config.toml
#[derive(Debug, Clone, PartialEq)]
pub struct CodexAuth {
    pub mode: AuthMode,
    pub(crate) api_key: String,
}

impl CodexAuth {
    /// Attempts to create CodexAuth from a config API key.
    /// Returns None if no API key is provided.
    pub fn from_config(api_key: Option<String>) -> Option<CodexAuth> {
        let api_key = api_key?.trim().to_string();
        if api_key.is_empty() {
            return None;
        }

        Some(Self {
            api_key,
            mode: AuthMode::ApiKey,
        })
    }

    /// Returns the API key for this auth.
    pub async fn get_token(&self) -> Result<String, std::io::Error> {
        Ok(self.api_key.clone())
    }

    /// Creates auth from an API key.
    pub fn from_api_key(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            mode: AuthMode::ApiKey,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_auth_from_api_key() {
        let auth = CodexAuth::from_api_key("sk-test-key");
        assert_eq!(auth.mode, AuthMode::ApiKey);
        assert_eq!(auth.api_key, "sk-test-key");
    }

    #[test]
    fn creates_auth_from_config() {
        let auth = CodexAuth::from_config(Some("sk-config-key".to_string()));
        assert_eq!(auth, Some(CodexAuth {
            api_key: "sk-config-key".to_string(),
            mode: AuthMode::ApiKey,
        }));
    }

    #[test]
    fn rejects_empty_config_api_key() {
        let auth = CodexAuth::from_config(Some("  ".to_string()));
        assert_eq!(auth, None);
    }

    #[test]
    fn rejects_none_config_api_key() {
        let auth = CodexAuth::from_config(None);
        assert_eq!(auth, None);
    }
}

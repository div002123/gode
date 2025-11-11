use std::env;

use codex_app_server_protocol::AuthMode;
use serde::{Deserialize, Serialize};

/// Mode for storing CLI authentication credentials.
/// Note: This is kept for backward compatibility but is not actively used
/// since authentication is now read directly from environment variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthCredentialsStoreMode {
    /// Store credentials in a file in the Codex home directory.
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
/// Authentication is read directly from environment variables.
#[derive(Debug, Clone, PartialEq)]
pub struct CodexAuth {
    pub mode: AuthMode,
    pub(crate) api_key: String,
}

impl CodexAuth {
    /// Attempts to create CodexAuth from environment variables.
    /// Returns None if no API key is available.
    pub fn from_env() -> Option<CodexAuth> {
        // Try OPENAI_API_KEY or CODEX_API_KEY from environment
        let api_key = read_openai_api_key_from_env()
            .or_else(read_codex_api_key_from_env)?;

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

pub const OPENAI_API_KEY_ENV_VAR: &str = "OPENAI_API_KEY";
pub const CODEX_API_KEY_ENV_VAR: &str = "CODEX_API_KEY";

pub fn read_openai_api_key_from_env() -> Option<String> {
    env::var(OPENAI_API_KEY_ENV_VAR)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn read_codex_api_key_from_env() -> Option<String> {
    env::var(CODEX_API_KEY_ENV_VAR)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
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
    fn api_key_trimmed_and_validated() {
        let trimmed = read_openai_api_key_from_env();
        // This test just validates the parsing logic, actual key requires env var
        assert!(trimmed.is_none() || !trimmed.unwrap().is_empty());
    }
}

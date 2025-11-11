mod storage;

use std::env;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use codex_app_server_protocol::AuthMode;

pub use crate::auth::storage::{AuthCredentialsStoreMode, AuthDotJson};
use crate::auth::storage::create_auth_storage;

/// Simple API Key based authentication.
#[derive(Debug, Clone, PartialEq)]
pub struct CodexAuth {
    pub mode: AuthMode,
    pub(crate) api_key: Option<String>,
}

impl CodexAuth {
    /// Loads the available auth information from auth storage.
    pub fn from_auth_storage(
        codex_home: &Path,
        auth_credentials_store_mode: AuthCredentialsStoreMode,
    ) -> std::io::Result<Option<CodexAuth>> {
        load_auth(codex_home, false, auth_credentials_store_mode)
    }

    /// Returns the API key for this auth.
    pub async fn get_token(&self) -> Result<String, std::io::Error> {
        Ok(self.api_key.clone().unwrap_or_default())
    }

    /// Creates auth from an API key.
    pub fn from_api_key(api_key: &str) -> Self {
        Self {
            api_key: Some(api_key.to_owned()),
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

/// Delete the auth.json file inside `codex_home` if it exists. Returns `Ok(true)`
/// if a file was removed, `Ok(false)` if no auth file was present.
pub fn logout(
    codex_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> std::io::Result<bool> {
    let storage = create_auth_storage(codex_home.to_path_buf(), auth_credentials_store_mode);
    storage.delete()
}

/// Writes an `auth.json` that contains only the API key.
pub fn login_with_api_key(
    codex_home: &Path,
    api_key: &str,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> std::io::Result<()> {
    let auth_dot_json = AuthDotJson {
        openai_api_key: Some(api_key.to_string()),
        tokens: None,
        last_refresh: None,
    };
    save_auth(codex_home, &auth_dot_json, auth_credentials_store_mode)
}

/// Persist the provided auth payload using the specified backend.
pub fn save_auth(
    codex_home: &Path,
    auth: &AuthDotJson,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> std::io::Result<()> {
    let storage = create_auth_storage(codex_home.to_path_buf(), auth_credentials_store_mode);
    storage.save(auth)
}

/// Load CLI auth data using the configured credential store backend.
/// Returns `None` when no credentials are stored.
pub fn load_auth_dot_json(
    codex_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> std::io::Result<Option<AuthDotJson>> {
    let storage = create_auth_storage(codex_home.to_path_buf(), auth_credentials_store_mode);
    storage.load()
}

fn load_auth(
    codex_home: &Path,
    enable_codex_api_key_env: bool,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> std::io::Result<Option<CodexAuth>> {
    if enable_codex_api_key_env && let Some(api_key) = read_codex_api_key_from_env() {
        return Ok(Some(CodexAuth::from_api_key(&api_key)));
    }

    let storage = create_auth_storage(codex_home.to_path_buf(), auth_credentials_store_mode);

    let auth_dot_json = match storage.load()? {
        Some(auth) => auth,
        None => return Ok(None),
    };

    let AuthDotJson {
        openai_api_key: auth_json_api_key,
        tokens: _,
        last_refresh: _,
    } = auth_dot_json;

    // Prefer AuthMode.ApiKey if it's set in the auth.json.
    if let Some(api_key) = &auth_json_api_key {
        return Ok(Some(CodexAuth::from_api_key(api_key)));
    }

    Ok(None)
}

/// Internal cached auth state.
#[derive(Clone, Debug)]
struct CachedAuth {
    auth: Option<CodexAuth>,
}

/// Central manager providing a single source of truth for auth.json derived
/// authentication data. It loads once (or on preference change) and then
/// hands out cloned `CodexAuth` values so the rest of the program has a
/// consistent snapshot.
#[derive(Debug)]
pub struct AuthManager {
    codex_home: PathBuf,
    inner: RwLock<CachedAuth>,
    enable_codex_api_key_env: bool,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
}

impl AuthManager {
    /// Create a new manager loading the initial auth using the provided
    /// preferred auth method. Errors loading auth are swallowed; `auth()` will
    /// simply return `None` in that case so callers can treat it as an
    /// unauthenticated state.
    pub fn new(
        codex_home: PathBuf,
        enable_codex_api_key_env: bool,
        auth_credentials_store_mode: AuthCredentialsStoreMode,
    ) -> Self {
        let auth = load_auth(
            &codex_home,
            enable_codex_api_key_env,
            auth_credentials_store_mode,
        )
        .ok()
        .flatten();
        Self {
            codex_home,
            inner: RwLock::new(CachedAuth { auth }),
            enable_codex_api_key_env,
            auth_credentials_store_mode,
        }
    }

    /// Create an AuthManager with a specific CodexAuth, for testing only.
    pub fn from_auth_for_testing(auth: CodexAuth) -> std::sync::Arc<Self> {
        let cached = CachedAuth { auth: Some(auth) };
        std::sync::Arc::new(Self {
            codex_home: PathBuf::new(),
            inner: RwLock::new(cached),
            enable_codex_api_key_env: false,
            auth_credentials_store_mode: AuthCredentialsStoreMode::File,
        })
    }

    /// Current cached auth (clone). May be `None` if not logged in or load failed.
    pub fn auth(&self) -> Option<CodexAuth> {
        self.inner.read().ok().and_then(|c| c.auth.clone())
    }

    /// Force a reload of the auth information from auth.json. Returns
    /// whether the auth value changed.
    pub fn reload(&self) -> bool {
        let new_auth = load_auth(
            &self.codex_home,
            self.enable_codex_api_key_env,
            self.auth_credentials_store_mode,
        )
        .ok()
        .flatten();
        if let Ok(mut guard) = self.inner.write() {
            let changed = !AuthManager::auths_equal(&guard.auth, &new_auth);
            guard.auth = new_auth;
            changed
        } else {
            false
        }
    }

    fn auths_equal(a: &Option<CodexAuth>, b: &Option<CodexAuth>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    /// Convenience constructor returning an `Arc` wrapper.
    pub fn shared(
        codex_home: PathBuf,
        enable_codex_api_key_env: bool,
        auth_credentials_store_mode: AuthCredentialsStoreMode,
    ) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self::new(
            codex_home,
            enable_codex_api_key_env,
            auth_credentials_store_mode,
        ))
    }

    /// Log out by deleting the on‑disk auth.json (if present). Returns Ok(true)
    /// if a file was removed, Ok(false) if no auth file existed. On success,
    /// reloads the in‑memory auth cache so callers immediately observe the
    /// unauthenticated state.
    pub fn logout(&self) -> std::io::Result<bool> {
        let removed = super::auth::logout(&self.codex_home, self.auth_credentials_store_mode)?;
        // Always reload to clear any cached auth (even if file absent).
        self.reload();
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn login_with_api_key_overwrites_existing_auth_json() {
        let dir = tempdir().unwrap();
        let auth_path = dir.path().join("auth.json");
        let stale_auth = serde_json::json!({
            "OPENAI_API_KEY": "sk-old",
            "tokens": null
        });
        std::fs::write(
            &auth_path,
            serde_json::to_string_pretty(&stale_auth).unwrap(),
        )
        .unwrap();

        super::login_with_api_key(dir.path(), "sk-new", AuthCredentialsStoreMode::File)
            .expect("login_with_api_key should succeed");

        let loaded = super::load_auth_dot_json(dir.path(), AuthCredentialsStoreMode::File)
            .expect("should load");
        assert_eq!(loaded.as_ref().unwrap().openai_api_key.as_deref(), Some("sk-new"));
        assert!(loaded.as_ref().unwrap().tokens.is_none());
    }

    #[test]
    fn missing_auth_json_returns_none() {
        let dir = tempdir().unwrap();
        let auth = CodexAuth::from_auth_storage(dir.path(), AuthCredentialsStoreMode::File)
            .expect("call should succeed");
        assert_eq!(auth, None);
    }

    #[test]
    fn loads_api_key_from_auth_json() {
        let dir = tempdir().unwrap();
        let auth_file = dir.path().join("auth.json");
        std::fs::write(
            auth_file,
            r#"{"OPENAI_API_KEY":"sk-test-key","tokens":null,"last_refresh":null}"#,
        )
        .unwrap();

        let auth = super::load_auth(dir.path(), false, AuthCredentialsStoreMode::File)
            .unwrap()
            .unwrap();
        assert_eq!(auth.mode, AuthMode::ApiKey);
        assert_eq!(auth.api_key, Some("sk-test-key".to_string()));
    }

    #[test]
    fn logout_removes_auth_file() -> Result<(), std::io::Error> {
        let dir = tempdir()?;
        let auth_dot_json = AuthDotJson {
            openai_api_key: Some("sk-test-key".to_string()),
            tokens: None,
            last_refresh: None,
        };
        super::save_auth(dir.path(), &auth_dot_json, AuthCredentialsStoreMode::File)?;
        let auth_file = dir.path().join("auth.json");
        assert!(auth_file.exists());
        assert!(logout(dir.path(), AuthCredentialsStoreMode::File)?);
        assert!(!auth_file.exists());
        Ok(())
    }
}

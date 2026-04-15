//! AI integration: daimon client, hoosh inference.
//!
//! Provides the standard AGNOS daimon/hoosh client pattern for the hisab agent.
//! Requires the `ai` feature.

use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::error::DaimonError;

/// Agent name used when registering with daimon.
pub const AGENT_NAME: &str = "hisab";

/// Default daimon API URL.
pub const DEFAULT_DAIMON_URL: &str = "http://localhost:8090";

/// Default hoosh (LLM gateway) API URL.
pub const DEFAULT_HOOSH_URL: &str = "http://localhost:8088";

/// Registration request sent to daimon.
#[derive(Debug, Serialize)]
struct RegisterRequest {
    name: String,
    capabilities: Vec<String>,
}

/// Registration response from daimon.
#[derive(Debug, Deserialize)]
struct RegisterResponse {
    id: String,
}

/// Daimon client for the hisab agent.
pub struct DaimonClient {
    client: reqwest::Client,
    daimon_url: String,
    hoosh_url: String,
    agent_id: Option<String>,
}

impl DaimonClient {
    /// Create a new client with default URLs.
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            daimon_url: DEFAULT_DAIMON_URL.to_string(),
            hoosh_url: DEFAULT_HOOSH_URL.to_string(),
            agent_id: None,
        }
    }

    /// Create a new client with custom URLs.
    #[must_use]
    pub fn with_urls(daimon_url: &str, hoosh_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            daimon_url: daimon_url.to_string(),
            hoosh_url: hoosh_url.to_string(),
            agent_id: None,
        }
    }

    /// The agent ID assigned by daimon after registration.
    #[must_use]
    pub fn agent_id(&self) -> Option<&str> {
        self.agent_id.as_deref()
    }

    /// Register with the daimon agent runtime.
    ///
    /// # Errors
    ///
    /// Returns [`DaimonError::Http`] if the HTTP request fails.
    /// Returns [`DaimonError::Registration`] if the server rejects registration.
    pub async fn register(&mut self) -> Result<String, DaimonError> {
        let url = format!("{}/v1/agents/register", self.daimon_url);
        let body = RegisterRequest {
            name: AGENT_NAME.to_string(),
            capabilities: vec![
                "math".to_string(),
                "geometry".to_string(),
                "calculus".to_string(),
                "numerical-methods".to_string(),
            ],
        };

        debug!(agent = AGENT_NAME, url = %url, "registering with daimon");

        let resp = self.client.post(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(DaimonError::Registration(format!("HTTP {status}: {text}")));
        }

        let reg: RegisterResponse = resp.json().await?;
        self.agent_id = Some(reg.id.clone());
        debug!(agent_id = %reg.id, "registered with daimon");
        Ok(reg.id)
    }

    /// Send a heartbeat to daimon.
    ///
    /// # Errors
    ///
    /// Returns [`DaimonError::Heartbeat`] if the client is not registered or the server rejects the heartbeat.
    /// Returns [`DaimonError::Http`] if the HTTP request fails.
    pub async fn heartbeat(&self) -> Result<(), DaimonError> {
        let id = self.agent_id.as_deref().ok_or_else(|| {
            DaimonError::Heartbeat("not registered — call register() first".to_string())
        })?;

        let url = format!("{}/v1/agents/{}/heartbeat", self.daimon_url, id);
        let resp = self.client.post(&url).send().await?;

        if !resp.status().is_success() {
            warn!(agent_id = %id, status = %resp.status(), "heartbeat failed");
            return Err(DaimonError::Heartbeat(format!("HTTP {}", resp.status())));
        }

        Ok(())
    }

    /// Query hoosh (LLM gateway) with a prompt.
    ///
    /// # Errors
    ///
    /// Returns [`DaimonError::Http`] if the HTTP request fails.
    /// Returns [`DaimonError::HooshQuery`] if the server returns a non-success status.
    pub async fn hoosh_query(&self, prompt: &str) -> Result<String, DaimonError> {
        let url = format!("{}/v1/chat/completions", self.hoosh_url);

        let body = serde_json::json!({
            "model": "default",
            "messages": [
                {"role": "system", "content": "You are a mathematical assistant."},
                {"role": "user", "content": prompt}
            ]
        });

        let resp = self.client.post(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            return Err(DaimonError::HooshQuery(format!("HTTP {}", resp.status())));
        }

        let json: serde_json::Value = resp.json().await?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(content)
    }
}

impl Default for DaimonClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_default_urls() {
        let client = DaimonClient::new();
        assert_eq!(client.daimon_url, DEFAULT_DAIMON_URL);
        assert_eq!(client.hoosh_url, DEFAULT_HOOSH_URL);
        assert!(client.agent_id().is_none());
    }

    #[test]
    fn client_custom_urls() {
        let client = DaimonClient::with_urls("http://daimon:9090", "http://hoosh:9088");
        assert_eq!(client.daimon_url, "http://daimon:9090");
        assert_eq!(client.hoosh_url, "http://hoosh:9088");
    }

    #[test]
    fn client_default_trait() {
        let client = DaimonClient::default();
        assert_eq!(client.daimon_url, DEFAULT_DAIMON_URL);
        assert_eq!(client.hoosh_url, DEFAULT_HOOSH_URL);
        assert!(client.agent_id().is_none());
    }

    #[test]
    fn error_display_variants() {
        let e = DaimonError::Registration("bad token".to_string());
        assert_eq!(e.to_string(), "registration failed: bad token");

        let e = DaimonError::Heartbeat("timeout".to_string());
        assert_eq!(e.to_string(), "heartbeat failed: timeout");

        let e = DaimonError::HooshQuery("model not found".to_string());
        assert_eq!(e.to_string(), "hoosh query failed: model not found");
    }

    #[tokio::test]
    async fn heartbeat_without_registration_fails() {
        let client = DaimonClient::new();
        let result = client.heartbeat().await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, DaimonError::Heartbeat(_)));
    }

    #[test]
    fn agent_id_none_before_registration() {
        let client = DaimonClient::new();
        assert!(client.agent_id().is_none());
    }

    #[test]
    fn constants_are_correct() {
        assert_eq!(AGENT_NAME, "hisab");
        assert!(DEFAULT_DAIMON_URL.starts_with("http://"));
        assert!(DEFAULT_HOOSH_URL.starts_with("http://"));
    }
}

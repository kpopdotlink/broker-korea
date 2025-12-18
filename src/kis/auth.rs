//! KIS API Authentication
//!
//! Handles OAuth token acquisition and hashkey generation.

use super::types::{KisConfig, KisError, KisResult};
use crate::http::HttpClient;
use serde::{Deserialize, Serialize};

/// Token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    /// Token expiration timestamp (Unix seconds)
    pub expires_at: u64,
}

impl TokenInfo {
    /// Check if token is expired (with 5 minute buffer)
    pub fn is_expired(&self) -> bool {
        let now = current_timestamp();
        self.expires_at <= now + 300 // 5 minute buffer
    }
}

/// Token response from KIS API
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    access_token_token_expired: Option<String>,
}

/// Get access token
pub fn get_token(http: &HttpClient, config: &KisConfig) -> KisResult<TokenInfo> {
    let body = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": config.app_key,
        "appsecret": config.app_secret
    });

    let response = http.post_json("/oauth2/tokenP", &body, None);

    if !response.is_success() {
        return Err(KisError::Auth(format!(
            "Token request failed: {} - {}",
            response.status,
            response.error.unwrap_or(response.body)
        )));
    }

    let token_response: TokenResponse = response
        .json()
        .map_err(|e| KisError::Parse(format!("Failed to parse token response: {}", e)))?;

    let now = current_timestamp();
    let expires_at = now + token_response.expires_in;

    Ok(TokenInfo {
        access_token: token_response.access_token,
        token_type: token_response.token_type,
        expires_in: token_response.expires_in,
        expires_at,
    })
}

/// Hashkey request body
#[derive(Debug, Serialize)]
struct HashkeyRequest<'a, T: Serialize> {
    #[serde(flatten)]
    body: &'a T,
}

/// Hashkey response
#[derive(Debug, Deserialize)]
struct HashkeyResponse {
    #[serde(rename = "BODY")]
    body: Option<HashkeyBody>,
    #[serde(rename = "HASH")]
    hash: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HashkeyBody {
    #[serde(rename = "HASH")]
    hash: String,
}

/// Get hashkey for order requests
///
/// Hashkey is required for POST requests that modify data (orders, etc.)
pub fn get_hashkey<T: Serialize>(
    http: &HttpClient,
    config: &KisConfig,
    body: &T,
) -> KisResult<String> {
    let mut headers = std::collections::HashMap::new();
    headers.insert("appkey".to_string(), config.app_key.clone());
    headers.insert("appsecret".to_string(), config.app_secret.clone());
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let response = http.post_json("/uapi/hashkey", body, Some(headers));

    if !response.is_success() {
        return Err(KisError::Auth(format!(
            "Hashkey request failed: {} - {}",
            response.status,
            response.error.unwrap_or(response.body)
        )));
    }

    let hashkey_response: HashkeyResponse = response
        .json()
        .map_err(|e| KisError::Parse(format!("Failed to parse hashkey response: {}", e)))?;

    // Try both possible response formats
    if let Some(hash) = hashkey_response.hash {
        return Ok(hash);
    }
    if let Some(body) = hashkey_response.body {
        return Ok(body.hash);
    }

    Err(KisError::Parse("Hashkey not found in response".to_string()))
}

/// WebSocket approval key response
#[derive(Debug, Deserialize)]
struct ApprovalKeyResponse {
    approval_key: String,
}

/// Get WebSocket approval key
pub fn get_websocket_key(http: &HttpClient, config: &KisConfig) -> KisResult<String> {
    let body = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": config.app_key,
        "appsecret": config.app_secret
    });

    let response = http.post_json("/oauth2/Approval", &body, None);

    if !response.is_success() {
        return Err(KisError::Auth(format!(
            "WebSocket approval request failed: {} - {}",
            response.status,
            response.error.unwrap_or(response.body)
        )));
    }

    let approval_response: ApprovalKeyResponse = response
        .json()
        .map_err(|e| KisError::Parse(format!("Failed to parse approval response: {}", e)))?;

    Ok(approval_response.approval_key)
}

/// Get current Unix timestamp in seconds
fn current_timestamp() -> u64 {
    // In WASM environment, we don't have access to system time easily
    // This is a simplified version - in production, you might want to
    // get time from the host or use a monotonic counter
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

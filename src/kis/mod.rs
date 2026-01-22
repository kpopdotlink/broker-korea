//! Korea Investment & Securities (KIS) Open API Client
//!
//! This module provides a comprehensive interface to KIS Open API,
//! supporting:
//! - Domestic stocks (국내주식)
//! - Overseas stocks (해외주식)
//! - Domestic futures/options (국내선물옵션)
//! - Overseas futures/options (해외선물옵션)
//! - Domestic bonds (장내채권)

pub mod auth;
pub mod bond;
pub mod domestic_future;
pub mod domestic_stock;
pub mod overseas_future;
pub mod overseas_stock;
pub mod types;

use crate::http::{HttpClient, HttpResponse};
use auth::TokenInfo;
use std::collections::HashMap;
use types::{KisConfig, KisError, KisResult};

/// KIS API Base URLs
pub const PROD_BASE_URL: &str = "https://openapi.koreainvestment.com:9443";
pub const VPS_BASE_URL: &str = "https://openapivts.koreainvestment.com:29443";

/// Main KIS API Client
pub struct KisClient {
    config: KisConfig,
    http: HttpClient,
    token: Option<TokenInfo>,
}

impl KisClient {
    /// Create a new KIS client
    pub fn new(config: KisConfig) -> Self {
        let base_url = if config.is_paper {
            VPS_BASE_URL
        } else {
            PROD_BASE_URL
        };

        let http = HttpClient::new(base_url)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("Accept", "text/plain")
            .header("charset", "UTF-8");

        Self {
            config,
            http,
            token: None,
        }
    }

    /// Get account number (8 digits)
    pub fn cano(&self) -> &str {
        &self.config.account_no[..8]
    }

    /// Get account product code (2 digits)
    pub fn acnt_prdt_cd(&self) -> &str {
        &self.config.account_no[8..10]
    }

    /// Check if authenticated
    pub fn is_authenticated(&self) -> bool {
        self.token
            .as_ref()
            .map(|t| !t.is_expired())
            .unwrap_or(false)
    }

    /// Ensure authenticated, auto-refresh if needed
    pub fn ensure_auth(&mut self) -> KisResult<()> {
        if !self.is_authenticated() {
            self.authenticate()?;
        }
        Ok(())
    }

    /// Authenticate and get access token
    pub fn authenticate(&mut self) -> KisResult<()> {
        let token = auth::get_token(&self.http, &self.config)?;
        self.token = Some(token);
        Ok(())
    }

    /// Get current access token
    pub fn access_token(&self) -> Option<&str> {
        self.token.as_ref().map(|t| t.access_token.as_str())
    }

    /// Build common headers for API requests
    pub fn build_headers(&self, tr_id: &str) -> KisResult<HashMap<String, String>> {
        let token = self.token.as_ref().ok_or_else(|| {
            KisError::Auth("Not authenticated. Call authenticate() first.".to_string())
        })?;

        let mut headers = HashMap::new();
        headers.insert(
            "authorization".to_string(),
            format!("Bearer {}", token.access_token),
        );
        headers.insert("appkey".to_string(), self.config.app_key.clone());
        headers.insert("appsecret".to_string(), self.config.app_secret.clone());
        headers.insert("tr_id".to_string(), tr_id.to_string());
        headers.insert("custtype".to_string(), "P".to_string()); // 개인

        Ok(headers)
    }

    /// Make authenticated GET request
    pub fn get(&self, path: &str, tr_id: &str, query: Option<&str>) -> KisResult<HttpResponse> {
        let headers = self.build_headers(tr_id)?;
        let url = if let Some(q) = query {
            format!("{}?{}", path, q)
        } else {
            path.to_string()
        };

        let response = self.http.get(&url, Some(headers));

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        Ok(response)
    }

    /// Make authenticated POST request
    pub fn post<T: serde::Serialize>(
        &self,
        path: &str,
        tr_id: &str,
        body: &T,
    ) -> KisResult<HttpResponse> {
        let headers = self.build_headers(tr_id)?;
        let response = self.http.post_json(path, body, Some(headers));

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        Ok(response)
    }

    /// Get hashkey for order requests
    pub fn get_hashkey<T: serde::Serialize>(&self, body: &T) -> KisResult<String> {
        auth::get_hashkey(&self.http, &self.config, body)
    }
}

/// Environment mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Environment {
    /// Production (실전투자)
    Production,
    /// Virtual/Paper trading (모의투자)
    Paper,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Production => PROD_BASE_URL,
            Environment::Paper => VPS_BASE_URL,
        }
    }

    pub fn tr_id_prefix(&self) -> &'static str {
        match self {
            Environment::Production => "T",
            Environment::Paper => "V",
        }
    }
}

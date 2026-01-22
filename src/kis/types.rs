//! Common types for KIS API

use serde::{Deserialize, Serialize};

/// KIS API Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KisConfig {
    /// App Key from KIS Developers
    pub app_key: String,
    /// App Secret from KIS Developers
    pub app_secret: String,
    /// Account number (10 digits: CANO 8자리 + ACNT_PRDT_CD 2자리)
    pub account_no: String,
    /// Whether this is paper trading (모의투자)
    pub is_paper: bool,
}

impl KisConfig {
    pub fn new(app_key: String, app_secret: String, account_no: String, is_paper: bool) -> Self {
        Self {
            app_key,
            app_secret,
            account_no,
            is_paper,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.app_key.is_empty() {
            return Err("app_key is required".to_string());
        }
        if self.app_secret.is_empty() {
            return Err("app_secret is required".to_string());
        }
        if self.account_no.len() != 10 {
            return Err("account_no must be 10 digits".to_string());
        }
        Ok(())
    }
}

/// KIS API Error
#[derive(Debug, Clone)]
pub enum KisError {
    /// Authentication error
    Auth(String),
    /// API error with code and message
    Api { code: String, message: String },
    /// Network error
    Network(String),
    /// Parse error
    Parse(String),
    /// Validation error
    Validation(String),
}

impl std::fmt::Display for KisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KisError::Auth(msg) => write!(f, "Auth error: {}", msg),
            KisError::Api { code, message } => write!(f, "API error [{}]: {}", code, message),
            KisError::Network(msg) => write!(f, "Network error: {}", msg),
            KisError::Parse(msg) => write!(f, "Parse error: {}", msg),
            KisError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

/// Result type for KIS API operations
pub type KisResult<T> = Result<T, KisError>;

/// Common API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KisApiResponse<T> {
    /// Return code (0 = success)
    pub rt_cd: String,
    /// Message code
    pub msg_cd: String,
    /// Message
    pub msg1: String,
    /// Response body
    pub output: Option<T>,
    /// Output for list responses
    pub output1: Option<Vec<T>>,
    pub output2: Option<serde_json::Value>,
}

impl<T> KisApiResponse<T> {
    pub fn is_success(&self) -> bool {
        self.rt_cd == "0"
    }
}

/// Order side
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OrderSide {
    /// 매수
    Buy,
    /// 매도
    Sell,
}

impl OrderSide {
    /// Get KIS API order division code for domestic stocks
    pub fn domestic_code(&self) -> &'static str {
        match self {
            OrderSide::Buy => "00",  // 지정가 매수
            OrderSide::Sell => "00", // 지정가 매도
        }
    }
}

/// Order type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    /// 지정가
    Limit,
    /// 시장가
    Market,
    /// 조건부지정가
    ConditionalLimit,
    /// 최유리지정가
    BestLimit,
    /// 최우선지정가
    PriorityLimit,
    /// 장전시간외
    PreMarket,
    /// 장후시간외
    AfterMarket,
}

impl OrderType {
    /// Get KIS API order division code
    pub fn code(&self) -> &'static str {
        match self {
            OrderType::Limit => "00",
            OrderType::Market => "01",
            OrderType::ConditionalLimit => "02",
            OrderType::BestLimit => "03",
            OrderType::PriorityLimit => "04",
            OrderType::PreMarket => "05",
            OrderType::AfterMarket => "06",
        }
    }
}

/// Market/Exchange code for overseas
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Exchange {
    // 미국
    NYSE,
    NASDAQ,
    AMEX,
    // 홍콩
    SEHK,
    // 중국
    SHAA, // 상해
    SZAA, // 심천
    // 일본
    TKSE,
    // 베트남
    HASE,
    VNSE,
}

impl Exchange {
    pub fn code(&self) -> &'static str {
        match self {
            Exchange::NYSE => "NYSE",
            Exchange::NASDAQ => "NASD",
            Exchange::AMEX => "AMEX",
            Exchange::SEHK => "SEHK",
            Exchange::SHAA => "SHAA",
            Exchange::SZAA => "SZAA",
            Exchange::TKSE => "TKSE",
            Exchange::HASE => "HASE",
            Exchange::VNSE => "VNSE",
        }
    }

    /// Check if this is a US exchange
    pub fn is_us(&self) -> bool {
        matches!(self, Exchange::NYSE | Exchange::NASDAQ | Exchange::AMEX)
    }
}

/// Position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub symbol_name: String,
    pub quantity: f64,
    pub average_price: f64,
    pub current_price: f64,
    pub profit_loss: f64,
    pub profit_loss_rate: f64,
    pub market_value: f64,
}

/// Account balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub total_equity: f64,
    pub total_deposit: f64,
    pub available_cash: f64,
    pub total_profit_loss: f64,
    pub total_profit_loss_rate: f64,
}

/// Order execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResult {
    /// Order number (주문번호)
    pub order_no: String,
    /// Order date (주문일자)
    pub order_date: String,
    /// Success flag
    pub success: bool,
    /// Message
    pub message: String,
}

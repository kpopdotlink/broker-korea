//! broker-korea: Korean securities broker integration plugin
//!
//! This plugin provides integration with Korea Investment & Securities (KIS) broker.
//! Implements the standard broker plugin interface.

mod http;
mod kis;

use chrono::Utc;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::slice;
use std::sync::Mutex;

use kis::types::KisConfig;
use kis::KisClient;
use models::order::{Order, OrderSide, OrderStatus, OrderType};
use models::portfolio::{AccountBalance, AccountSummary, Position};
use plugin_api::{
    GetAccountsRequest, GetAccountsResponse, GetPositionsRequest, GetPositionsResponse,
    SubmitOrderRequest, SubmitOrderResponse,
};

// --- State Management ---

struct BrokerState {
    client: Option<KisClient>,
    account_no: String,
    is_paper: bool,
    orders: HashMap<String, Order>,
    next_order_id: u64,
}

impl BrokerState {
    fn new() -> Self {
        Self {
            client: None,
            account_no: String::new(),
            is_paper: true,
            orders: HashMap::new(),
            next_order_id: 1,
        }
    }
}

lazy_static! {
    static ref STATE: Mutex<BrokerState> = Mutex::new(BrokerState::new());
}

// --- Error Response Helper ---

fn error_response<T: serde::Serialize + Default>(error: &str) -> u64 {
    // Log error (will be visible in host logs)
    eprintln!("[broker-korea] Error: {}", error);

    // Return default response with error in extensions
    let response = T::default();
    serialize_response(&response)
}

// --- WASM Exports ---

/// Memory allocation for host communication
#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {
    let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as usize as i32
}

/// Initialize plugin with configuration
#[no_mangle]
pub extern "C" fn initialize(ptr: i32, len: i32) -> u64 {
    let config_json: serde_json::Value = parse_request(ptr, len);

    let mut state = STATE.lock().unwrap();

    // Parse configuration from secrets
    let app_key = config_json
        .get("app_key")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let app_secret = config_json
        .get("app_secret")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let account_no = config_json
        .get("account_no")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let is_paper = config_json
        .get("is_paper")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    // Validate configuration
    if app_key.is_empty() || app_secret.is_empty() || account_no.is_empty() {
        return serialize_response(&serde_json::json!({
            "success": false,
            "error": "Missing required configuration: app_key, app_secret, or account_no"
        }));
    }

    if account_no.len() != 10 {
        return serialize_response(&serde_json::json!({
            "success": false,
            "error": "account_no must be 10 digits (CANO 8 + ACNT_PRDT_CD 2)"
        }));
    }

    // Create KIS configuration
    let kis_config = KisConfig::new(app_key, app_secret, account_no.clone(), is_paper);

    // Create KIS client
    let client = KisClient::new(kis_config);

    state.client = Some(client);
    state.account_no = account_no;
    state.is_paper = is_paper;

    serialize_response(&serde_json::json!({
        "success": true,
        "message": format!("Initialized KIS broker ({})", if is_paper { "paper" } else { "production" })
    }))
}

/// Get available accounts with real balance from KIS API
#[no_mangle]
pub extern "C" fn get_accounts(ptr: i32, len: i32) -> u64 {
    let _req: GetAccountsRequest = parse_request(ptr, len);

    let mut state = STATE.lock().unwrap();

    let client = match state.client.as_mut() {
        Some(c) => c,
        None => {
            return serialize_response(&GetAccountsResponse {
                accounts: vec![create_error_account("Plugin not initialized")],
            });
        }
    };

    // Fetch real balance from KIS API
    let (balance, positions) = match client.domestic_balance() {
        Ok((items, summary)) => {
            // Convert KIS balance to our format
            let total_equity: f64 = summary
                .tot_evlu_amt
                .as_ref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);
            let available_cash: f64 = summary
                .ord_psbl_cash
                .as_ref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);
            let deposit: f64 = summary
                .dnca_tot_amt
                .as_ref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);

            let balance = AccountBalance {
                currency: "KRW".to_string(),
                total_equity,
                available_cash,
                buying_power: available_cash,
                locked_cash: deposit - available_cash,
            };

            // Convert positions
            let positions: Vec<Position> = items
                .iter()
                .filter_map(|item| {
                    let symbol = item.pdno.as_ref()?.clone();
                    let quantity: f64 = item.hldg_qty.as_ref()?.parse().ok()?;
                    let avg_price: f64 = item.pchs_avg_pric.as_ref()?.parse().ok()?;
                    let current_price: f64 = item.prpr.as_ref()?.parse().ok()?;
                    let pnl: f64 = item.evlu_pfls_amt.as_ref()?.parse().ok()?;
                    let pnl_rate: f64 = item.evlu_pfls_rt.as_ref()?.parse().ok()?;

                    Some(Position {
                        symbol_id: symbol,
                        quantity,
                        average_price: avg_price,
                        current_price,
                        unrealized_pnl: pnl,
                        unrealized_pnl_percent: pnl_rate,
                    })
                })
                .collect();

            (balance, positions)
        }
        Err(e) => {
            eprintln!("[broker-korea] Failed to fetch balance: {}", e);
            // Return empty balance on error
            (
                AccountBalance {
                    currency: "KRW".to_string(),
                    total_equity: 0.0,
                    available_cash: 0.0,
                    buying_power: 0.0,
                    locked_cash: 0.0,
                },
                vec![],
            )
        }
    };

    let account = AccountSummary {
        id: state.account_no.clone(),
        name: format!(
            "KIS {} Account",
            if state.is_paper { "Paper" } else { "Live" }
        ),
        broker_id: "broker-korea".to_string(),
        is_paper: state.is_paper,
        balance,
        positions,
        updated_at: Utc::now(),
        extensions: None,
    };

    let response = GetAccountsResponse {
        accounts: vec![account],
    };

    serialize_response(&response)
}

/// Get positions for an account
#[no_mangle]
pub extern "C" fn get_positions(ptr: i32, len: i32) -> u64 {
    let req: GetPositionsRequest = parse_request(ptr, len);

    let mut state = STATE.lock().unwrap();

    // Copy account_no before borrowing client
    let account_no = state.account_no.clone();

    // Verify account ID matches
    if req.account_id != account_no {
        return serialize_response(&GetPositionsResponse { positions: vec![] });
    }

    let client = match state.client.as_mut() {
        Some(c) => c,
        None => {
            return serialize_response(&GetPositionsResponse { positions: vec![] });
        }
    };

    // Fetch positions from KIS API
    let positions = match client.domestic_balance() {
        Ok((items, _summary)) => items
            .iter()
            .filter_map(|item| {
                let symbol = item.pdno.as_ref()?.clone();
                let quantity: f64 = item.hldg_qty.as_ref()?.parse().ok()?;
                let avg_price: f64 = item.pchs_avg_pric.as_ref()?.parse().ok()?;
                let current_price: f64 = item.prpr.as_ref()?.parse().ok()?;
                let pnl: f64 = item.evlu_pfls_amt.as_ref()?.parse().ok()?;
                let pnl_rate: f64 = item.evlu_pfls_rt.as_ref()?.parse().ok()?;

                Some(Position {
                    symbol_id: symbol,
                    quantity,
                    average_price: avg_price,
                    current_price,
                    unrealized_pnl: pnl,
                    unrealized_pnl_percent: pnl_rate,
                })
            })
            .collect(),
        Err(e) => {
            eprintln!("[broker-korea] Failed to fetch positions: {}", e);
            vec![]
        }
    };

    let response = GetPositionsResponse { positions };

    serialize_response(&response)
}

/// Submit an order to KIS broker
#[no_mangle]
pub extern "C" fn submit_order(ptr: i32, len: i32) -> u64 {
    let req: SubmitOrderRequest = parse_request(ptr, len);
    let mut state = STATE.lock().unwrap();

    let client = match state.client.as_mut() {
        Some(c) => c,
        None => {
            return serialize_response(&SubmitOrderResponse {
                order: create_error_order(&req, "Plugin not initialized"),
            });
        }
    };

    // Extract order details
    let symbol = &req.order.symbol_id;
    let quantity = req.order.quantity as u32;
    let side = &req.order.side;
    let order_type = &req.order.order_type;
    let limit_price = req.order.limit_price.unwrap_or(0.0) as u32;

    // Map order type to KIS order type
    let kis_order_type = match order_type {
        OrderType::Market => kis::types::OrderType::Market,
        OrderType::Limit => kis::types::OrderType::Limit,
        _ => kis::types::OrderType::Limit, // Default to limit for unsupported types
    };

    // Submit order to KIS API
    let result = match side {
        OrderSide::Buy => client.domestic_buy(symbol, quantity, limit_price, kis_order_type),
        OrderSide::Sell => client.domestic_sell(symbol, quantity, limit_price, kis_order_type),
    };

    match result {
        Ok(kis_response) => {
            // Generate local order ID combining KIS order number
            let order_id = kis_response
                .odno
                .unwrap_or_else(|| format!("kr_{}", state.next_order_id));
            state.next_order_id += 1;

            let order = Order {
                id: order_id.clone(),
                request: req.order.clone(),
                status: OrderStatus::Submitted,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                average_filled_price: None,
                filled_quantity: 0.0,
                extensions: Some({
                    let mut map = HashMap::new();
                    if let Some(time) = kis_response.ord_tmd {
                        map.insert(
                            "kis_order_time".to_string(),
                            serde_json::Value::String(time),
                        );
                    }
                    map
                }),
                persona_id: req.order.persona_id.clone(),
            };

            state.orders.insert(order_id, order.clone());

            let response = SubmitOrderResponse { order };
            serialize_response(&response)
        }
        Err(e) => {
            eprintln!("[broker-korea] Order failed: {}", e);
            serialize_response(&SubmitOrderResponse {
                order: create_error_order(&req, &format!("Order failed: {}", e)),
            })
        }
    }
}

// --- Helper Functions ---

fn parse_request<T: serde::de::DeserializeOwned>(ptr: i32, len: i32) -> T {
    let slice = unsafe { slice::from_raw_parts(ptr as *const u8, len as usize) };
    serde_json::from_slice(slice).expect("Failed to parse request")
}

fn serialize_response<T: serde::Serialize>(response: &T) -> u64 {
    let res_bytes = serde_json::to_vec(response).expect("Failed to serialize response");

    let out_len = res_bytes.len() as i32;
    let out_ptr = alloc(out_len);

    unsafe {
        std::ptr::copy_nonoverlapping(res_bytes.as_ptr(), out_ptr as *mut u8, out_len as usize);
    }

    ((out_ptr as u64) << 32) | (out_len as u64)
}

fn create_error_account(error: &str) -> AccountSummary {
    AccountSummary {
        id: "error".to_string(),
        name: format!("Error: {}", error),
        broker_id: "broker-korea".to_string(),
        is_paper: true,
        balance: AccountBalance {
            currency: "KRW".to_string(),
            total_equity: 0.0,
            available_cash: 0.0,
            buying_power: 0.0,
            locked_cash: 0.0,
        },
        positions: vec![],
        updated_at: Utc::now(),
        extensions: None,
    }
}

fn create_error_order(req: &SubmitOrderRequest, error: &str) -> Order {
    Order {
        id: format!("error_{}", Utc::now().timestamp_millis()),
        request: req.order.clone(),
        status: OrderStatus::Rejected,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        average_filled_price: None,
        filled_quantity: 0.0,
        extensions: Some({
            let mut map = HashMap::new();
            map.insert("error".to_string(), serde_json::Value::String(error.to_string()));
            map
        }),
        persona_id: req.order.persona_id.clone(),
    }
}

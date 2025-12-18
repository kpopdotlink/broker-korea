//! broker-korea: Korean securities broker integration plugin
//!
//! This plugin provides integration with Korean securities brokers.
//! Implements the standard broker plugin interface.

mod http;
mod kis;

use chrono::Utc;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::slice;
use std::sync::Mutex;

use models::order::{Order, OrderStatus};
use models::portfolio::{AccountBalance, AccountSummary, Position};
use plugin_api::{
    GetAccountsRequest, GetAccountsResponse, GetPositionsRequest, GetPositionsResponse,
    SubmitOrderRequest, SubmitOrderResponse,
};

// --- Configuration ---

/// Broker configuration loaded from secrets
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
struct BrokerConfig {
    app_key: String,
    app_secret: String,
    account_no: String,
    is_paper: bool,
}

// --- State Management ---

struct BrokerState {
    config: Option<BrokerConfig>,
    accounts: Vec<AccountSummary>,
    positions: HashMap<String, Vec<Position>>,
    orders: HashMap<String, Order>,
    next_order_id: u64,
}

impl BrokerState {
    fn new() -> Self {
        Self {
            config: None,
            accounts: vec![],
            positions: HashMap::new(),
            orders: HashMap::new(),
            next_order_id: 1,
        }
    }
}

lazy_static! {
    static ref STATE: Mutex<BrokerState> = Mutex::new(BrokerState::new());
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
    let config = BrokerConfig {
        app_key: config_json
            .get("app_key")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string(),
        app_secret: config_json
            .get("app_secret")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string(),
        account_no: config_json
            .get("account_no")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string(),
        is_paper: config_json
            .get("is_paper")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
    };

    // Setup default account
    let account = AccountSummary {
        id: config.account_no.clone(),
        name: "Korea Broker Account".to_string(),
        broker_id: "broker-korea".to_string(),
        is_paper: config.is_paper,
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
    };

    state.accounts = vec![account];
    state.positions.insert(config.account_no.clone(), vec![]);
    state.config = Some(config);

    serialize_response(&serde_json::json!({ "success": true }))
}

/// Get available accounts
#[no_mangle]
pub extern "C" fn get_accounts(ptr: i32, len: i32) -> u64 {
    let _req: GetAccountsRequest = parse_request(ptr, len);

    let state = STATE.lock().unwrap();

    // TODO: Fetch real account info from broker API
    let response = GetAccountsResponse {
        accounts: state.accounts.clone(),
    };

    serialize_response(&response)
}

/// Get positions for an account
#[no_mangle]
pub extern "C" fn get_positions(ptr: i32, len: i32) -> u64 {
    let req: GetPositionsRequest = parse_request(ptr, len);

    let state = STATE.lock().unwrap();

    // TODO: Fetch real positions from broker API
    let positions = state
        .positions
        .get(&req.account_id)
        .cloned()
        .unwrap_or_default();

    let response = GetPositionsResponse { positions };

    serialize_response(&response)
}

/// Submit an order
#[no_mangle]
pub extern "C" fn submit_order(ptr: i32, len: i32) -> u64 {
    let req: SubmitOrderRequest = parse_request(ptr, len);
    let mut state = STATE.lock().unwrap();

    // Generate local order ID
    let order_id = format!("kr_{}", state.next_order_id);
    state.next_order_id += 1;

    let order = Order {
        id: order_id.clone(),
        request: req.order.clone(),
        status: OrderStatus::Created,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        average_filled_price: None,
        filled_quantity: 0.0,
        extensions: None,
        persona_id: req.order.persona_id.clone(),
    };

    // TODO: Submit order to actual broker API
    // For now, just store locally

    state.orders.insert(order_id, order.clone());

    let response = SubmitOrderResponse { order };

    serialize_response(&response)
}

// --- Helpers ---

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

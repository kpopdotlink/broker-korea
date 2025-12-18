//! Overseas Futures/Options API (해외선물옵션)
//!
//! This module provides API implementations for overseas futures/options trading
//! including order placement, modification, cancellation, position inquiry,
//! margin inquiry, and execution history.

use super::{
    types::{KisApiResponse, KisError, KisResult, OrderResult},
    KisClient,
};
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants
// ============================================================================

/// TR_ID codes for overseas futures/options
pub mod tr_id {
    /// 신규매수 (New Buy)
    pub const NEW_BUY: &str = "OTFM3001U";
    /// 신규매도 (New Sell)
    pub const NEW_SELL: &str = "OTFM3002U";
    /// 청산매수 (Close Buy)
    pub const CLOSE_BUY: &str = "OTFM3003U";
    /// 청산매도 (Close Sell)
    pub const CLOSE_SELL: &str = "OTFM3004U";
    /// 정정/취소 (Modify/Cancel)
    pub const MODIFY_CANCEL: &str = "OTFM3005U";
    /// 잔고/미결제 조회 (Inquire Unsettled Positions)
    pub const INQUIRE_UNSETTLED: &str = "OTFM3304R";
    /// 증거금 조회 (Inquire Deposit)
    pub const INQUIRE_DEPOSIT: &str = "OTFM3306R";
    /// 체결내역 조회 (Inquire Execution)
    pub const INQUIRE_EXECUTION: &str = "OTFM3307R";
}

// ============================================================================
// Request Types
// ============================================================================

/// Order request for overseas futures/options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasFutureOrderRequest {
    /// 계좌번호 8자리
    #[serde(rename = "CANO")]
    pub cano: String,

    /// 계좌상품코드 (08 for futures/options)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,

    /// 해외선물상품번호 (종목코드)
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub symbol: String,

    /// 매도매수구분코드 (01: 매도, 02: 매수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub side: String,

    /// 가격구분코드 (1: 지정가, 2: 시장가)
    #[serde(rename = "PRIC_DVSN_CD")]
    pub price_type: String,

    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub quantity: String,

    /// 선물옵션지정가격
    #[serde(rename = "FUOP_LIMT_PRIC")]
    pub limit_price: String,
}

impl OverseasFutureOrderRequest {
    /// Create a new order request
    pub fn new(
        cano: String,
        acnt_prdt_cd: String,
        symbol: String,
        side: OrderSide,
        price_type: PriceType,
        quantity: u32,
        limit_price: f64,
    ) -> Self {
        Self {
            cano,
            acnt_prdt_cd,
            symbol,
            side: side.code().to_string(),
            price_type: price_type.code().to_string(),
            quantity: quantity.to_string(),
            limit_price: limit_price.to_string(),
        }
    }
}

/// Modify/Cancel request for overseas futures/options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasFutureModifyCancelRequest {
    /// 계좌번호 8자리
    #[serde(rename = "CANO")]
    pub cano: String,

    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,

    /// 원주문번호
    #[serde(rename = "ORGN_ODNO")]
    pub original_order_no: String,

    /// 정정취소구분코드 (01: 정정, 02: 취소)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub modify_cancel_type: String,

    /// 주문수량 (정정 시)
    #[serde(rename = "ORD_QTY", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// 지정가격 (정정 시)
    #[serde(rename = "FUOP_LIMT_PRIC", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
}

impl OverseasFutureModifyCancelRequest {
    /// Create a cancel request
    pub fn cancel(cano: String, acnt_prdt_cd: String, original_order_no: String) -> Self {
        Self {
            cano,
            acnt_prdt_cd,
            original_order_no,
            modify_cancel_type: "02".to_string(),
            quantity: None,
            limit_price: None,
        }
    }

    /// Create a modify request
    pub fn modify(
        cano: String,
        acnt_prdt_cd: String,
        original_order_no: String,
        quantity: Option<u32>,
        limit_price: Option<f64>,
    ) -> Self {
        Self {
            cano,
            acnt_prdt_cd,
            original_order_no,
            modify_cancel_type: "01".to_string(),
            quantity: quantity.map(|q| q.to_string()),
            limit_price: limit_price.map(|p| p.to_string()),
        }
    }
}

// ============================================================================
// Response Types
// ============================================================================

/// Order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasFutureOrderResponse {
    /// 주문번호
    #[serde(rename = "ODNO")]
    pub order_no: String,

    /// 주문시각
    #[serde(rename = "ORD_TMD")]
    pub order_time: String,

    /// Return code (추가 메시지 정보)
    #[serde(rename = "KRX_FWDG_ORD_ORGNO", default)]
    pub extra_info: String,
}

/// Unsettled position (미결제 포지션)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsettledPosition {
    /// 종목코드
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub symbol: String,

    /// 종목명
    #[serde(rename = "OVRS_FUTR_FX_ITEM_NM")]
    pub symbol_name: String,

    /// 미결제수량
    #[serde(rename = "UNPD_QTY")]
    pub quantity: String,

    /// 평균가
    #[serde(rename = "AVG_PRIC")]
    pub average_price: String,

    /// 현재가
    #[serde(rename = "PRPR")]
    pub current_price: String,

    /// 평가손익
    #[serde(rename = "EVLU_PFLS_AMT")]
    pub profit_loss: String,

    /// 평가손익률
    #[serde(rename = "EVLU_PFLS_RT")]
    pub profit_loss_rate: String,

    /// 매수매도구분 (01: 매도, 02: 매수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub side: String,

    /// 통화코드
    #[serde(rename = "CRCY_CD")]
    pub currency: String,
}

/// Deposit information (증거금 정보)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositInfo {
    /// 총예탁금
    #[serde(rename = "TOT_DPSIT_AMT")]
    pub total_deposit: String,

    /// 주문가능금액
    #[serde(rename = "ORD_PSBL_AMT")]
    pub available_amount: String,

    /// 증거금
    #[serde(rename = "MGNA_AMT")]
    pub margin_amount: String,

    /// 평가손익
    #[serde(rename = "EVLU_PFLS_AMT")]
    pub profit_loss: String,

    /// 통화코드
    #[serde(rename = "CRCY_CD")]
    pub currency: String,

    /// 환산예탁금총액
    #[serde(rename = "FRCR_DPSIT_TOT_AMT")]
    pub foreign_deposit_total: String,
}

/// Execution record (체결내역)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRecord {
    /// 주문번호
    #[serde(rename = "ODNO")]
    pub order_no: String,

    /// 종목코드
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub symbol: String,

    /// 종목명
    #[serde(rename = "OVRS_FUTR_FX_ITEM_NM")]
    pub symbol_name: String,

    /// 체결수량
    #[serde(rename = "CCLD_QTY")]
    pub executed_quantity: String,

    /// 체결단가
    #[serde(rename = "CCLD_UNPR")]
    pub executed_price: String,

    /// 체결금액
    #[serde(rename = "CCLD_AMT")]
    pub executed_amount: String,

    /// 체결시각
    #[serde(rename = "CCLD_TMD")]
    pub executed_time: String,

    /// 매도매수구분
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub side: String,

    /// 주문구분 (신규/청산)
    #[serde(rename = "ORD_DVSN")]
    pub order_type: String,
}

// ============================================================================
// Enums
// ============================================================================

/// Order side for futures/options
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OrderSide {
    /// 매도 (Sell)
    Sell,
    /// 매수 (Buy)
    Buy,
}

impl OrderSide {
    pub fn code(&self) -> &'static str {
        match self {
            OrderSide::Sell => "01",
            OrderSide::Buy => "02",
        }
    }
}

/// Position type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PositionType {
    /// 신규 (New Position)
    New,
    /// 청산 (Close Position)
    Close,
}

impl PositionType {
    /// Get TR_ID for buy order
    pub fn buy_tr_id(&self) -> &'static str {
        match self {
            PositionType::New => tr_id::NEW_BUY,
            PositionType::Close => tr_id::CLOSE_BUY,
        }
    }

    /// Get TR_ID for sell order
    pub fn sell_tr_id(&self) -> &'static str {
        match self {
            PositionType::New => tr_id::NEW_SELL,
            PositionType::Close => tr_id::CLOSE_SELL,
        }
    }
}

/// Price type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PriceType {
    /// 지정가 (Limit)
    Limit,
    /// 시장가 (Market)
    Market,
}

impl PriceType {
    pub fn code(&self) -> &'static str {
        match self {
            PriceType::Limit => "1",
            PriceType::Market => "2",
        }
    }
}

// ============================================================================
// API Implementation
// ============================================================================

impl KisClient {
    /// Place an overseas futures/options order
    ///
    /// # Arguments
    /// * `symbol` - 종목코드
    /// * `side` - 매도/매수 구분
    /// * `position_type` - 신규/청산 구분
    /// * `price_type` - 지정가/시장가 구분
    /// * `quantity` - 주문수량
    /// * `limit_price` - 지정가격 (시장가 시 0)
    pub fn overseas_future_order(
        &self,
        symbol: &str,
        side: OrderSide,
        position_type: PositionType,
        price_type: PriceType,
        quantity: u32,
        limit_price: f64,
    ) -> KisResult<OrderResult> {
        let tr_id = match side {
            OrderSide::Buy => position_type.buy_tr_id(),
            OrderSide::Sell => position_type.sell_tr_id(),
        };

        let request = OverseasFutureOrderRequest::new(
            self.cano().to_string(),
            self.acnt_prdt_cd().to_string(),
            symbol.to_string(),
            side,
            price_type,
            quantity,
            limit_price,
        );

        // Get hashkey for order
        let hashkey = self.get_hashkey(&request)?;
        let mut headers = self.build_headers(tr_id)?;
        headers.insert("hashkey".to_string(), hashkey);

        let response = self.http.post_json(
            "/uapi/overseas-futureoption/v1/trading/order",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<OverseasFutureOrderResponse> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse order response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        let output = api_response
            .output
            .ok_or_else(|| KisError::Parse("Order response missing output".to_string()))?;

        Ok(OrderResult {
            order_no: output.order_no,
            order_date: output.order_time,
            success: true,
            message: api_response.msg1,
        })
    }

    /// Cancel an overseas futures/options order
    ///
    /// # Arguments
    /// * `order_no` - 원주문번호
    pub fn overseas_future_cancel(&self, order_no: &str) -> KisResult<OrderResult> {
        let request = OverseasFutureModifyCancelRequest::cancel(
            self.cano().to_string(),
            self.acnt_prdt_cd().to_string(),
            order_no.to_string(),
        );

        self.overseas_future_modify_cancel_internal(request)
    }

    /// Modify an overseas futures/options order
    ///
    /// # Arguments
    /// * `order_no` - 원주문번호
    /// * `quantity` - 변경할 수량 (None이면 변경 안 함)
    /// * `limit_price` - 변경할 가격 (None이면 변경 안 함)
    pub fn overseas_future_modify(
        &self,
        order_no: &str,
        quantity: Option<u32>,
        limit_price: Option<f64>,
    ) -> KisResult<OrderResult> {
        let request = OverseasFutureModifyCancelRequest::modify(
            self.cano().to_string(),
            self.acnt_prdt_cd().to_string(),
            order_no.to_string(),
            quantity,
            limit_price,
        );

        self.overseas_future_modify_cancel_internal(request)
    }

    /// Internal method to handle modify/cancel requests
    fn overseas_future_modify_cancel_internal(
        &self,
        request: OverseasFutureModifyCancelRequest,
    ) -> KisResult<OrderResult> {
        let hashkey = self.get_hashkey(&request)?;
        let mut headers = self.build_headers(tr_id::MODIFY_CANCEL)?;
        headers.insert("hashkey".to_string(), hashkey);

        let response = self.http.post_json(
            "/uapi/overseas-futureoption/v1/trading/order-rvsecncl",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<OverseasFutureOrderResponse> =
            response.json().map_err(|e| {
                KisError::Parse(format!("Failed to parse modify/cancel response: {}", e))
            })?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        let output = api_response
            .output
            .ok_or_else(|| KisError::Parse("Modify/cancel response missing output".to_string()))?;

        Ok(OrderResult {
            order_no: output.order_no,
            order_date: output.order_time,
            success: true,
            message: api_response.msg1,
        })
    }

    /// Inquire unsettled positions (잔고/미결제 조회)
    pub fn overseas_future_inquire_unsettled(&self) -> KisResult<Vec<UnsettledPosition>> {
        let query = format!(
            "CANO={}&ACNT_PRDT_CD={}&OVRS_FUTR_FX_PDNO=&CTX_AREA_FK200=&CTX_AREA_NK200=",
            self.cano(),
            self.acnt_prdt_cd()
        );

        let response = self.get(
            "/uapi/overseas-futureoption/v1/trading/inquire-unpd",
            tr_id::INQUIRE_UNSETTLED,
            Some(&query),
        )?;

        let api_response: KisApiResponse<UnsettledPosition> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse unsettled positions: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        Ok(api_response.output1.unwrap_or_default())
    }

    /// Inquire deposit/margin information (증거금 조회)
    pub fn overseas_future_inquire_deposit(&self) -> KisResult<DepositInfo> {
        let query = format!(
            "CANO={}&ACNT_PRDT_CD={}&OVRS_FUTR_FX_PDNO=&WCRC_FRCR_DVSN_CD=01&NATN_CD=",
            self.cano(),
            self.acnt_prdt_cd()
        );

        let response = self.get(
            "/uapi/overseas-futureoption/v1/trading/inquire-deposit",
            tr_id::INQUIRE_DEPOSIT,
            Some(&query),
        )?;

        let api_response: KisApiResponse<DepositInfo> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse deposit info: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("Deposit response missing output".to_string()))
    }

    /// Inquire execution history (체결내역 조회)
    ///
    /// # Arguments
    /// * `start_date` - 조회시작일자 (YYYYMMDD)
    /// * `end_date` - 조회종료일자 (YYYYMMDD)
    pub fn overseas_future_inquire_execution(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> KisResult<Vec<ExecutionRecord>> {
        let query = format!(
            "CANO={}&ACNT_PRDT_CD={}&OVRS_FUTR_FX_PDNO=&STRT_DT={}&END_DT={}&SLL_BUY_DVSN_CD=&CCLD_NCCS_DVSN_CD=&SORT_SQN=DS&CTX_AREA_FK200=&CTX_AREA_NK200=",
            self.cano(),
            self.acnt_prdt_cd(),
            start_date,
            end_date
        );

        let response = self.get(
            "/uapi/overseas-futureoption/v1/trading/inquire-ccld",
            tr_id::INQUIRE_EXECUTION,
            Some(&query),
        )?;

        let api_response: KisApiResponse<ExecutionRecord> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse execution records: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        Ok(api_response.output1.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_side_code() {
        assert_eq!(OrderSide::Sell.code(), "01");
        assert_eq!(OrderSide::Buy.code(), "02");
    }

    #[test]
    fn test_price_type_code() {
        assert_eq!(PriceType::Limit.code(), "1");
        assert_eq!(PriceType::Market.code(), "2");
    }

    #[test]
    fn test_position_type_tr_id() {
        assert_eq!(PositionType::New.buy_tr_id(), "OTFM3001U");
        assert_eq!(PositionType::New.sell_tr_id(), "OTFM3002U");
        assert_eq!(PositionType::Close.buy_tr_id(), "OTFM3003U");
        assert_eq!(PositionType::Close.sell_tr_id(), "OTFM3004U");
    }
}

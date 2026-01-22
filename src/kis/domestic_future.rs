//! Domestic Futures/Options API (국내선물옵션)
//!
//! This module provides APIs for domestic futures and options trading through KIS.

use super::{
    types::{KisApiResponse, KisError, KisResult},
    KisClient,
};
use serde::{Deserialize, Serialize};

/// Futures/Options order action type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FutureOrderAction {
    /// 신규매수 (New Buy)
    NewBuy,
    /// 신규매도 (New Sell)
    NewSell,
    /// 청산매수 (Liquidate Buy)
    LiquidateBuy,
    /// 청산매도 (Liquidate Sell)
    LiquidateSell,
}

impl FutureOrderAction {
    /// Get TR_ID for production environment
    pub fn prod_tr_id(&self) -> &'static str {
        match self {
            FutureOrderAction::NewBuy => "TTTO0101U",
            FutureOrderAction::NewSell => "TTTO0102U",
            FutureOrderAction::LiquidateBuy => "TTTO0103U",
            FutureOrderAction::LiquidateSell => "TTTO0104U",
        }
    }

    /// Get TR_ID for paper trading environment
    pub fn paper_tr_id(&self) -> &'static str {
        match self {
            FutureOrderAction::NewBuy => "VTTO0101U",
            FutureOrderAction::NewSell => "VTTO0102U",
            FutureOrderAction::LiquidateBuy => "VTTO0103U",
            FutureOrderAction::LiquidateSell => "VTTO0104U",
        }
    }

    /// Get sell/buy division code
    pub fn sll_buy_dvsn_cd(&self) -> &'static str {
        match self {
            FutureOrderAction::NewBuy | FutureOrderAction::LiquidateBuy => "02", // 매수
            FutureOrderAction::NewSell | FutureOrderAction::LiquidateSell => "01", // 매도
        }
    }
}

/// Price type for futures/options orders
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FuturePriceType {
    /// 지정가 (Limit)
    Limit,
    /// 시장가 (Market)
    Market,
}

impl FuturePriceType {
    /// Get price type code
    pub fn code(&self) -> &'static str {
        match self {
            FuturePriceType::Limit => "1",
            FuturePriceType::Market => "2",
        }
    }
}

/// Futures/Options order request
#[derive(Debug, Clone, Serialize)]
pub struct FutureOrderRequest {
    /// 계좌번호 (Account number - 8 digits)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (Account product code)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목코드 (Symbol code, e.g., 101S3000)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드 (Sell/Buy division code: 01=매도, 02=매수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문수량 (Order quantity)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (Order price)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 호가유형코드 (Price type code: 1=지정가, 2=시장가)
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
}

/// Futures/Options order response
#[derive(Debug, Clone, Deserialize)]
pub struct FutureOrderResponse {
    /// 주문번호 (Order number)
    #[serde(rename = "ORD_NO")]
    pub ord_no: Option<String>,
    /// 주문시각 (Order time)
    #[serde(rename = "ORD_TMD")]
    pub ord_tmd: Option<String>,
}

/// Futures/Options order modify/cancel request
#[derive(Debug, Clone, Serialize)]
pub struct FutureModifyCancelRequest {
    /// 계좌번호 (Account number - 8 digits)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (Account product code)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문번호 (Original order number)
    #[serde(rename = "ORGN_ORD_NO")]
    pub orgn_ord_no: String,
    /// 정정취소구분코드 (Modify/Cancel division: 01=정정, 02=취소)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량 (Order quantity - for modify)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Option<String>,
    /// 주문단가 (Order price - for modify)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: Option<String>,
}

/// Futures/Options balance item
#[derive(Debug, Clone, Deserialize)]
pub struct FutureBalance {
    /// 종목코드 (Symbol code)
    #[serde(rename = "PDNO")]
    pub pdno: Option<String>,
    /// 종목명 (Symbol name)
    #[serde(rename = "PRDT_NAME")]
    pub prdt_name: Option<String>,
    /// 매도매수구분 (Sell/Buy division)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: Option<String>,
    /// 잔고수량 (Balance quantity)
    #[serde(rename = "CBLC_QTY")]
    pub cblc_qty: Option<String>,
    /// 평균단가 (Average price)
    #[serde(rename = "AVG_UNPR")]
    pub avg_unpr: Option<String>,
    /// 현재가 (Current price)
    #[serde(rename = "PRPR")]
    pub prpr: Option<String>,
    /// 평가손익 (Profit/Loss)
    #[serde(rename = "EVLU_PFLS_AMT")]
    pub evlu_pfls_amt: Option<String>,
    /// 수익률 (Profit/Loss rate)
    #[serde(rename = "PFLS_RT")]
    pub pfls_rt: Option<String>,
}

/// Futures/Options deposit information
#[derive(Debug, Clone, Deserialize)]
pub struct FutureDeposit {
    /// 예탁총액 (Total deposit)
    #[serde(rename = "DNCA_TOT_AMT")]
    pub dnca_tot_amt: Option<String>,
    /// 위탁증거금 (Margin)
    #[serde(rename = "MGNA_AMT")]
    pub mgna_amt: Option<String>,
    /// 위탁증거금률 (Margin ratio)
    #[serde(rename = "MGNA_RT")]
    pub mgna_rt: Option<String>,
    /// 주문가능금액 (Available amount for order)
    #[serde(rename = "ORD_PSBL_AMT")]
    pub ord_psbl_amt: Option<String>,
    /// 인출가능금액 (Withdrawable amount)
    #[serde(rename = "WDRW_PSBL_AMT")]
    pub wdrw_psbl_amt: Option<String>,
}

/// Futures/Options execution item
#[derive(Debug, Clone, Deserialize)]
pub struct FutureExecution {
    /// 주문번호 (Order number)
    #[serde(rename = "ORD_NO")]
    pub ord_no: Option<String>,
    /// 종목코드 (Symbol code)
    #[serde(rename = "PDNO")]
    pub pdno: Option<String>,
    /// 종목명 (Symbol name)
    #[serde(rename = "PRDT_NAME")]
    pub prdt_name: Option<String>,
    /// 매도매수구분 (Sell/Buy division)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: Option<String>,
    /// 체결수량 (Executed quantity)
    #[serde(rename = "CCLD_QTY")]
    pub ccld_qty: Option<String>,
    /// 체결단가 (Executed price)
    #[serde(rename = "CCLD_UNPR")]
    pub ccld_unpr: Option<String>,
    /// 체결시각 (Execution time)
    #[serde(rename = "CCLD_TMD")]
    pub ccld_tmd: Option<String>,
    /// 체결금액 (Executed amount)
    #[serde(rename = "CCLD_AMT")]
    pub ccld_amt: Option<String>,
}

impl KisClient {
    /// Get TR_ID based on environment (production or paper trading)
    fn get_future_tr_id<'a>(&self, prod_id: &'a str, paper_id: &'a str) -> &'a str {
        if self.config.is_paper {
            paper_id
        } else {
            prod_id
        }
    }

    /// Place a futures/options order
    ///
    /// # Arguments
    /// * `action` - Order action type (NewBuy, NewSell, LiquidateBuy, LiquidateSell)
    /// * `symbol` - Symbol code (e.g., "101S3000")
    /// * `quantity` - Order quantity
    /// * `price` - Order price (use "0" for market orders)
    /// * `price_type` - Price type (Limit or Market)
    ///
    /// # Returns
    /// Order response with order number and time
    pub fn future_order(
        &mut self,
        action: FutureOrderAction,
        symbol: &str,
        quantity: u32,
        price: &str,
        price_type: FuturePriceType,
    ) -> KisResult<FutureOrderResponse> {
        self.ensure_auth()?;

        let tr_id = self.get_future_tr_id(action.prod_tr_id(), action.paper_tr_id());

        let request = FutureOrderRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            pdno: symbol.to_string(),
            sll_buy_dvsn_cd: action.sll_buy_dvsn_cd().to_string(),
            ord_qty: quantity.to_string(),
            unit_price: price.to_string(),
            nmpr_type_cd: price_type.code().to_string(),
        };

        // Generate hashkey for order request
        let hashkey = self.get_hashkey(&request)?;

        // Build headers with hashkey
        let mut headers = self.build_headers(tr_id)?;
        headers.insert("hashkey".to_string(), hashkey);

        // POST request
        let response = self.http.post_json(
            "/uapi/domestic-futureoption/v1/trading/order",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<FutureOrderResponse> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse order response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("Missing output in order response".to_string()))
    }

    /// Modify a futures/options order
    ///
    /// # Arguments
    /// * `order_no` - Original order number to modify
    /// * `quantity` - New order quantity
    /// * `price` - New order price
    ///
    /// # Returns
    /// Success flag
    pub fn future_modify_order(
        &mut self,
        order_no: &str,
        quantity: u32,
        price: &str,
    ) -> KisResult<bool> {
        self.ensure_auth()?;

        let tr_id = self.get_future_tr_id("TTTO0105U", "VTTO0105U");

        let request = FutureModifyCancelRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            orgn_ord_no: order_no.to_string(),
            rvse_cncl_dvsn_cd: "01".to_string(), // 정정
            ord_qty: Some(quantity.to_string()),
            unit_price: Some(price.to_string()),
        };

        // Generate hashkey for modify request
        let hashkey = self.get_hashkey(&request)?;

        // Build headers with hashkey
        let mut headers = self.build_headers(tr_id)?;
        headers.insert("hashkey".to_string(), hashkey);

        // POST request
        let response = self.http.post_json(
            "/uapi/domestic-futureoption/v1/trading/order-rvsecncl",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<serde_json::Value> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse modify response: {}", e)))?;

        Ok(api_response.is_success())
    }

    /// Cancel a futures/options order
    ///
    /// # Arguments
    /// * `order_no` - Original order number to cancel
    ///
    /// # Returns
    /// Success flag
    pub fn future_cancel_order(&mut self, order_no: &str) -> KisResult<bool> {
        self.ensure_auth()?;

        let tr_id = self.get_future_tr_id("TTTO0106U", "VTTO0106U");

        let request = FutureModifyCancelRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            orgn_ord_no: order_no.to_string(),
            rvse_cncl_dvsn_cd: "02".to_string(), // 취소
            ord_qty: None,
            unit_price: None,
        };

        // Generate hashkey for cancel request
        let hashkey = self.get_hashkey(&request)?;

        // Build headers with hashkey
        let mut headers = self.build_headers(tr_id)?;
        headers.insert("hashkey".to_string(), hashkey);

        // POST request
        let response = self.http.post_json(
            "/uapi/domestic-futureoption/v1/trading/order-rvsecncl",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<serde_json::Value> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse cancel response: {}", e)))?;

        Ok(api_response.is_success())
    }

    /// Get futures/options balance
    ///
    /// # Returns
    /// List of futures/options positions
    pub fn future_get_balance(&mut self) -> KisResult<Vec<FutureBalance>> {
        self.ensure_auth()?;

        let tr_id = self.get_future_tr_id("TTTO5201R", "VTTO5201R");

        let query = format!(
            "CANO={}&ACNT_PRDT_CD={}&AFHR_FLPR_YN=N&INQR_DVSN=00&UNPR_DVSN=01&FUND_STTL_ICLD_YN=N&FNCG_AMT_AUTO_RDPT_YN=N&OFL_YN=N&CTX_AREA_FK100=&CTX_AREA_NK100=",
            self.cano(),
            self.acnt_prdt_cd()
        );

        let response = self.get(
            "/uapi/domestic-futureoption/v1/trading/inquire-balance",
            tr_id,
            Some(&query),
        )?;

        let api_response: KisApiResponse<FutureBalance> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse balance response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        Ok(api_response.output1.unwrap_or_default())
    }

    /// Get futures/options deposit information
    ///
    /// # Returns
    /// Deposit information including margin and available amounts
    pub fn future_get_deposit(&mut self) -> KisResult<FutureDeposit> {
        self.ensure_auth()?;

        let tr_id = self.get_future_tr_id("TTTO5300R", "VTTO5300R");

        let query = format!(
            "CANO={}&ACNT_PRDT_CD={}&INQR_DVSN_1=00&INQR_DVSN_2=00",
            self.cano(),
            self.acnt_prdt_cd()
        );

        let response = self.get(
            "/uapi/domestic-futureoption/v1/trading/inquire-deposit",
            tr_id,
            Some(&query),
        )?;

        let api_response: KisApiResponse<FutureDeposit> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse deposit response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("Missing output in deposit response".to_string()))
    }

    /// Get futures/options execution history
    ///
    /// # Arguments
    /// * `date` - Query date (YYYYMMDD format, None for today)
    ///
    /// # Returns
    /// List of executed orders
    pub fn future_get_executions(&mut self, date: Option<&str>) -> KisResult<Vec<FutureExecution>> {
        self.ensure_auth()?;

        let tr_id = self.get_future_tr_id("TTTO5107R", "VTTO5107R");

        let query_date = date.unwrap_or("");
        let query = format!(
            "CANO={}&ACNT_PRDT_CD={}&INQR_STRT_DT={}&INQR_END_DT={}&SLL_BUY_DVSN_CD=00&INQR_DVSN=00&PDNO=&CCLD_DVSN=00&ORD_GNO_BRNO=&ODNO=&INQR_DVSN_3=00&INQR_DVSN_1=&CTX_AREA_FK100=&CTX_AREA_NK100=",
            self.cano(),
            self.acnt_prdt_cd(),
            query_date,
            query_date
        );

        let response = self.get(
            "/uapi/domestic-futureoption/v1/trading/inquire-ccnl",
            tr_id,
            Some(&query),
        )?;

        let api_response: KisApiResponse<FutureExecution> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse execution response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        Ok(api_response.output1.unwrap_or_default())
    }

    /// Place a new buy order for futures/options
    pub fn future_new_buy(
        &mut self,
        symbol: &str,
        quantity: u32,
        price: &str,
        price_type: FuturePriceType,
    ) -> KisResult<FutureOrderResponse> {
        self.future_order(
            FutureOrderAction::NewBuy,
            symbol,
            quantity,
            price,
            price_type,
        )
    }

    /// Place a new sell order for futures/options
    pub fn future_new_sell(
        &mut self,
        symbol: &str,
        quantity: u32,
        price: &str,
        price_type: FuturePriceType,
    ) -> KisResult<FutureOrderResponse> {
        self.future_order(
            FutureOrderAction::NewSell,
            symbol,
            quantity,
            price,
            price_type,
        )
    }

    /// Place a liquidate buy order for futures/options
    pub fn future_liquidate_buy(
        &mut self,
        symbol: &str,
        quantity: u32,
        price: &str,
        price_type: FuturePriceType,
    ) -> KisResult<FutureOrderResponse> {
        self.future_order(
            FutureOrderAction::LiquidateBuy,
            symbol,
            quantity,
            price,
            price_type,
        )
    }

    /// Place a liquidate sell order for futures/options
    pub fn future_liquidate_sell(
        &mut self,
        symbol: &str,
        quantity: u32,
        price: &str,
        price_type: FuturePriceType,
    ) -> KisResult<FutureOrderResponse> {
        self.future_order(
            FutureOrderAction::LiquidateSell,
            symbol,
            quantity,
            price,
            price_type,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_order_action_tr_ids() {
        assert_eq!(FutureOrderAction::NewBuy.prod_tr_id(), "TTTO0101U");
        assert_eq!(FutureOrderAction::NewSell.prod_tr_id(), "TTTO0102U");
        assert_eq!(FutureOrderAction::LiquidateBuy.prod_tr_id(), "TTTO0103U");
        assert_eq!(FutureOrderAction::LiquidateSell.prod_tr_id(), "TTTO0104U");

        assert_eq!(FutureOrderAction::NewBuy.paper_tr_id(), "VTTO0101U");
        assert_eq!(FutureOrderAction::NewSell.paper_tr_id(), "VTTO0102U");
        assert_eq!(FutureOrderAction::LiquidateBuy.paper_tr_id(), "VTTO0103U");
        assert_eq!(FutureOrderAction::LiquidateSell.paper_tr_id(), "VTTO0104U");
    }

    #[test]
    fn test_future_price_type_codes() {
        assert_eq!(FuturePriceType::Limit.code(), "1");
        assert_eq!(FuturePriceType::Market.code(), "2");
    }

    #[test]
    fn test_sell_buy_division_codes() {
        assert_eq!(FutureOrderAction::NewBuy.sll_buy_dvsn_cd(), "02");
        assert_eq!(FutureOrderAction::NewSell.sll_buy_dvsn_cd(), "01");
        assert_eq!(FutureOrderAction::LiquidateBuy.sll_buy_dvsn_cd(), "02");
        assert_eq!(FutureOrderAction::LiquidateSell.sll_buy_dvsn_cd(), "01");
    }
}

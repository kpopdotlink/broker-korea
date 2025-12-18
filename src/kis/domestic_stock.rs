//! 국내주식 API (Domestic Stock API)
//!
//! Provides functions for:
//! - Cash buy/sell orders (현금 매수/매도)
//! - Order modification/cancellation (정정/취소)
//! - Balance inquiry (잔고조회)
//! - Current price inquiry (현재가 조회)

use super::{
    types::{KisApiResponse, KisError, KisResult, OrderType},
    KisClient,
};
use serde::{Deserialize, Serialize};

/// 현금 매수/매도 주문 요청
#[derive(Debug, Clone, Serialize)]
pub struct CashOrderRequest {
    /// 계좌번호 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌번호 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목코드 (예: 005930)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문구분 (00:지정가, 01:시장가, 02:조건부지정가, 03:최유리, 04:최우선, 05:장전시간외, 06:장후시간외)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (시장가일 경우 0)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
}

/// 현금 매수/매도 주문 응답
#[derive(Debug, Clone, Deserialize)]
pub struct CashOrderResponse {
    /// 주문번호
    #[serde(rename = "ODNO")]
    pub odno: Option<String>,
    /// 주문시각
    #[serde(rename = "ORD_TMD")]
    pub ord_tmd: Option<String>,
}

/// 정정/취소 주문 요청
#[derive(Debug, Clone, Serialize)]
pub struct OrderReviseRequest {
    /// 계좌번호 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌번호 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 한국거래소전송주문조직번호 (공백)
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    /// 원주문번호
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문구분 (00:지정가)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 정정취소구분 (01:정정, 02:취소)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량 (정정시)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (정정시)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 잔량전부주문여부 (Y/N)
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
}

/// 정정/취소 주문 응답
#[derive(Debug, Clone, Deserialize)]
pub struct OrderReviseResponse {
    /// 주문번호
    #[serde(rename = "ODNO")]
    pub odno: Option<String>,
    /// 주문시각
    #[serde(rename = "ORD_TMD")]
    pub ord_tmd: Option<String>,
}

/// 잔고 조회 응답 (output1 항목)
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceItem {
    /// 종목코드
    #[serde(rename = "pdno")]
    pub pdno: Option<String>,
    /// 종목명
    #[serde(rename = "prdt_name")]
    pub prdt_name: Option<String>,
    /// 보유수량
    #[serde(rename = "hldg_qty")]
    pub hldg_qty: Option<String>,
    /// 매입평균가격
    #[serde(rename = "pchs_avg_pric")]
    pub pchs_avg_pric: Option<String>,
    /// 현재가
    #[serde(rename = "prpr")]
    pub prpr: Option<String>,
    /// 평가손익
    #[serde(rename = "evlu_pfls_amt")]
    pub evlu_pfls_amt: Option<String>,
    /// 평가손익율
    #[serde(rename = "evlu_pfls_rt")]
    pub evlu_pfls_rt: Option<String>,
    /// 평가금액
    #[serde(rename = "evlu_amt")]
    pub evlu_amt: Option<String>,
}

/// 잔고 조회 응답 (output2 합계)
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceSummary {
    /// 총평가금액
    #[serde(rename = "tot_evlu_amt")]
    pub tot_evlu_amt: Option<String>,
    /// 예수금총액
    #[serde(rename = "dnca_tot_amt")]
    pub dnca_tot_amt: Option<String>,
    /// 주문가능현금
    #[serde(rename = "ord_psbl_cash")]
    pub ord_psbl_cash: Option<String>,
    /// 총평가손익
    #[serde(rename = "evlu_pfls_smtl_amt")]
    pub evlu_pfls_smtl_amt: Option<String>,
    /// 총평가손익율
    #[serde(rename = "evlu_pfls_rt")]
    pub evlu_pfls_rt: Option<String>,
}

/// 잔고 조회 전체 응답
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceResponse {
    /// 잔고 목록
    pub output1: Option<Vec<BalanceItem>>,
    /// 잔고 합계
    pub output2: Option<BalanceSummary>,
}

/// 현재가 조회 응답
#[derive(Debug, Clone, Deserialize)]
pub struct CurrentPriceResponse {
    /// 종목코드
    #[serde(rename = "stck_shrn_iscd")]
    pub stck_shrn_iscd: Option<String>,
    /// 현재가
    #[serde(rename = "stck_prpr")]
    pub stck_prpr: Option<String>,
    /// 전일대비
    #[serde(rename = "prdy_vrss")]
    pub prdy_vrss: Option<String>,
    /// 전일대비율
    #[serde(rename = "prdy_ctrt")]
    pub prdy_ctrt: Option<String>,
    /// 누적거래량
    #[serde(rename = "acml_vol")]
    pub acml_vol: Option<String>,
    /// 거래대금
    #[serde(rename = "acml_tr_pbmn")]
    pub acml_tr_pbmn: Option<String>,
    /// 시가
    #[serde(rename = "stck_oprc")]
    pub stck_oprc: Option<String>,
    /// 고가
    #[serde(rename = "stck_hgpr")]
    pub stck_hgpr: Option<String>,
    /// 저가
    #[serde(rename = "stck_lwpr")]
    pub stck_lwpr: Option<String>,
}

impl KisClient {
    /// 현금 매수 주문
    ///
    /// # Arguments
    /// * `symbol` - 종목코드 (예: "005930")
    /// * `quantity` - 주문수량
    /// * `price` - 주문단가 (시장가일 경우 0)
    /// * `order_type` - 주문구분
    pub fn domestic_buy(
        &mut self,
        symbol: &str,
        quantity: u32,
        price: u32,
        order_type: OrderType,
    ) -> KisResult<CashOrderResponse> {
        self.ensure_auth()?;

        let request = CashOrderRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            pdno: symbol.to_string(),
            ord_dvsn: order_type.code().to_string(),
            ord_qty: quantity.to_string(),
            ord_unpr: price.to_string(),
        };

        let tr_id = if self.config.is_paper {
            "VTTC0802U" // 모의투자 매수
        } else {
            "TTTC0802U" // 실전투자 매수
        };

        // Get hashkey for POST order
        let hashkey = self.get_hashkey(&request)?;

        let mut headers = self.build_headers(tr_id)?;
        headers.insert("hashkey".to_string(), hashkey);

        let response = self.http.post_json(
            "/uapi/domestic-stock/v1/trading/order-cash",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<CashOrderResponse> = response
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
            .ok_or_else(|| KisError::Parse("No output in order response".to_string()))
    }

    /// 현금 매도 주문
    ///
    /// # Arguments
    /// * `symbol` - 종목코드 (예: "005930")
    /// * `quantity` - 주문수량
    /// * `price` - 주문단가 (시장가일 경우 0)
    /// * `order_type` - 주문구분
    pub fn domestic_sell(
        &mut self,
        symbol: &str,
        quantity: u32,
        price: u32,
        order_type: OrderType,
    ) -> KisResult<CashOrderResponse> {
        self.ensure_auth()?;

        let request = CashOrderRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            pdno: symbol.to_string(),
            ord_dvsn: order_type.code().to_string(),
            ord_qty: quantity.to_string(),
            ord_unpr: price.to_string(),
        };

        let tr_id = if self.config.is_paper {
            "VTTC0801U" // 모의투자 매도
        } else {
            "TTTC0801U" // 실전투자 매도
        };

        // Get hashkey for POST order
        let hashkey = self.get_hashkey(&request)?;

        let mut headers = self.build_headers(tr_id)?;
        headers.insert("hashkey".to_string(), hashkey);

        let response = self.http.post_json(
            "/uapi/domestic-stock/v1/trading/order-cash",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<CashOrderResponse> = response
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
            .ok_or_else(|| KisError::Parse("No output in order response".to_string()))
    }

    /// 주문 정정
    ///
    /// # Arguments
    /// * `order_no` - 원주문번호
    /// * `quantity` - 정정 주문수량
    /// * `price` - 정정 주문단가
    /// * `order_type` - 주문구분
    pub fn domestic_revise_order(
        &mut self,
        order_no: &str,
        quantity: u32,
        price: u32,
        order_type: OrderType,
    ) -> KisResult<OrderReviseResponse> {
        self.ensure_auth()?;

        let request = OrderReviseRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            krx_fwdg_ord_orgno: "".to_string(),
            orgn_odno: order_no.to_string(),
            ord_dvsn: order_type.code().to_string(),
            rvse_cncl_dvsn_cd: "01".to_string(), // 정정
            ord_qty: quantity.to_string(),
            ord_unpr: price.to_string(),
            qty_all_ord_yn: "N".to_string(),
        };

        let tr_id = "TTTC0803U"; // 정정취소 (실전/모의 동일)

        // Get hashkey for POST order
        let hashkey = self.get_hashkey(&request)?;

        let mut headers = self.build_headers(tr_id)?;
        headers.insert("hashkey".to_string(), hashkey);

        let response = self.http.post_json(
            "/uapi/domestic-stock/v1/trading/order-rvsecncl",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<OrderReviseResponse> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse revise response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("No output in revise response".to_string()))
    }

    /// 주문 취소
    ///
    /// # Arguments
    /// * `order_no` - 원주문번호
    /// * `quantity` - 취소 수량 (0이면 전량 취소)
    pub fn domestic_cancel_order(
        &mut self,
        order_no: &str,
        quantity: u32,
    ) -> KisResult<OrderReviseResponse> {
        self.ensure_auth()?;

        let qty_all_ord_yn = if quantity == 0 { "Y" } else { "N" };
        let ord_qty = if quantity == 0 {
            "0".to_string()
        } else {
            quantity.to_string()
        };

        let request = OrderReviseRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            krx_fwdg_ord_orgno: "".to_string(),
            orgn_odno: order_no.to_string(),
            ord_dvsn: "00".to_string(),          // 지정가
            rvse_cncl_dvsn_cd: "02".to_string(), // 취소
            ord_qty,
            ord_unpr: "0".to_string(),
            qty_all_ord_yn: qty_all_ord_yn.to_string(),
        };

        let tr_id = "TTTC0803U"; // 정정취소 (실전/모의 동일)

        // Get hashkey for POST order
        let hashkey = self.get_hashkey(&request)?;

        let mut headers = self.build_headers(tr_id)?;
        headers.insert("hashkey".to_string(), hashkey);

        let response = self.http.post_json(
            "/uapi/domestic-stock/v1/trading/order-rvsecncl",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<OrderReviseResponse> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse cancel response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("No output in cancel response".to_string()))
    }

    /// 잔고 조회
    ///
    /// # Returns
    /// 보유 종목 목록과 계좌 합계 정보
    pub fn domestic_balance(&mut self) -> KisResult<(Vec<BalanceItem>, BalanceSummary)> {
        self.ensure_auth()?;

        let tr_id = if self.config.is_paper {
            "VTTC8434R" // 모의투자
        } else {
            "TTTC8434R" // 실전투자
        };

        let query = format!(
            "CANO={}&ACNT_PRDT_CD={}&AFHR_FLPR_YN=N&INQR_DVSN=02&UNPR_DVSN=01&FUND_STTL_ICLD_YN=N&FNCG_AMT_AUTO_RDPT_YN=N&PRCS_DVSN=00&CTX_AREA_FK100=&CTX_AREA_NK100=",
            self.cano(),
            self.acnt_prdt_cd()
        );

        let response = self.get(
            "/uapi/domestic-stock/v1/trading/inquire-balance",
            tr_id,
            Some(&query),
        )?;

        #[derive(Debug, Deserialize)]
        struct BalanceApiResponse {
            rt_cd: String,
            msg_cd: String,
            msg1: String,
            output1: Option<Vec<BalanceItem>>,
            output2: Option<Vec<BalanceSummary>>,
        }

        let api_response: BalanceApiResponse = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse balance response: {}", e)))?;

        if api_response.rt_cd != "0" {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        let items = api_response.output1.unwrap_or_default();
        let summary = api_response
            .output2
            .and_then(|mut v| {
                if v.is_empty() {
                    None
                } else {
                    Some(v.remove(0))
                }
            })
            .unwrap_or_else(|| BalanceSummary {
                tot_evlu_amt: None,
                dnca_tot_amt: None,
                ord_psbl_cash: None,
                evlu_pfls_smtl_amt: None,
                evlu_pfls_rt: None,
            });

        Ok((items, summary))
    }

    /// 현재가 조회
    ///
    /// # Arguments
    /// * `symbol` - 종목코드 (예: "005930")
    ///
    /// # Returns
    /// 현재가 정보
    pub fn domestic_current_price(&mut self, symbol: &str) -> KisResult<CurrentPriceResponse> {
        self.ensure_auth()?;

        let tr_id = "FHKST01010100";

        let query = format!("FID_COND_MRKT_DIV_CODE=J&FID_INPUT_ISCD={}", symbol);

        let response = self.get(
            "/uapi/domestic-stock/v1/quotations/inquire-price",
            tr_id,
            Some(&query),
        )?;

        let api_response: KisApiResponse<CurrentPriceResponse> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse price response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("No output in price response".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cash_order_request_serialization() {
        let request = CashOrderRequest {
            cano: "12345678".to_string(),
            acnt_prdt_cd: "01".to_string(),
            pdno: "005930".to_string(),
            ord_dvsn: "00".to_string(),
            ord_qty: "10".to_string(),
            ord_unpr: "70000".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"CANO\":\"12345678\""));
        assert!(json.contains("\"PDNO\":\"005930\""));
    }

    #[test]
    fn test_order_type_codes() {
        assert_eq!(OrderType::Limit.code(), "00");
        assert_eq!(OrderType::Market.code(), "01");
        assert_eq!(OrderType::ConditionalLimit.code(), "02");
        assert_eq!(OrderType::BestLimit.code(), "03");
        assert_eq!(OrderType::PriorityLimit.code(), "04");
        assert_eq!(OrderType::PreMarket.code(), "05");
        assert_eq!(OrderType::AfterMarket.code(), "06");
    }
}

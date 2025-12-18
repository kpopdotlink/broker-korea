//! KIS API - 장내채권 (Domestic Bond)
//!
//! 국내 장내채권 거래 및 조회 기능을 제공합니다.
//! - 매수/매도 주문
//! - 주문 정정/취소
//! - 잔고 조회
//! - 호가 조회
//! - 현재가 조회

use super::types::{KisApiResponse, KisError, KisResult};
use super::KisClient;
use serde::{Deserialize, Serialize};

// ============================================================================
// Request/Response Types
// ============================================================================

/// 채권 매수 요청
#[derive(Debug, Clone, Serialize)]
pub struct BondBuyRequest {
    /// 계좌번호 (8자리)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (2자리)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 채권일련번호
    #[serde(rename = "BOND_SRNO")]
    pub bond_srno: String,
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가
    #[serde(rename = "ORD_PRIC")]
    pub ord_pric: String,
}

/// 채권 매도 요청
#[derive(Debug, Clone, Serialize)]
pub struct BondSellRequest {
    /// 계좌번호 (8자리)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (2자리)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 채권일련번호
    #[serde(rename = "BOND_SRNO")]
    pub bond_srno: String,
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가
    #[serde(rename = "ORD_PRIC")]
    pub ord_pric: String,
}

/// 채권 주문 정정/취소 요청
#[derive(Debug, Clone, Serialize)]
pub struct BondOrderReviseRequest {
    /// 계좌번호 (8자리)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (2자리)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 한국거래소전송주문조직번호
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    /// 주문번호
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문구분 (01: 정정, 02: 취소)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 정정수량 (취소시 0)
    #[serde(rename = "RVSE_QTY")]
    pub rvse_qty: Option<String>,
    /// 정정단가 (취소시 생략 가능)
    #[serde(rename = "RVSE_PRIC")]
    pub rvse_pric: Option<String>,
}

/// 채권 주문 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondOrderResponse {
    /// 한국거래소전송주문조직번호
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: Option<String>,
    /// 주문번호
    #[serde(rename = "ODNO")]
    pub odno: Option<String>,
    /// 주문시각
    #[serde(rename = "ORD_TMD")]
    pub ord_tmd: Option<String>,
}

/// 채권 잔고 조회 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondBalance {
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: Option<String>,
    /// 상품명
    #[serde(rename = "PRDT_NAME")]
    pub prdt_name: Option<String>,
    /// 보유수량
    #[serde(rename = "HLDG_QTY")]
    pub hldg_qty: Option<String>,
    /// 매입평균가액
    #[serde(rename = "PCHS_AVG_PRIC")]
    pub pchs_avg_pric: Option<String>,
    /// 매입금액
    #[serde(rename = "PCHS_AMT")]
    pub pchs_amt: Option<String>,
    /// 현재가
    #[serde(rename = "PRPR")]
    pub prpr: Option<String>,
    /// 평가금액
    #[serde(rename = "EVLU_AMT")]
    pub evlu_amt: Option<String>,
    /// 평가손익금액
    #[serde(rename = "EVLU_PFLS_AMT")]
    pub evlu_pfls_amt: Option<String>,
    /// 평가손익율
    #[serde(rename = "EVLU_PFLS_RT")]
    pub evlu_pfls_rt: Option<String>,
    /// 만기일자
    #[serde(rename = "EXPR_DT")]
    pub expr_dt: Option<String>,
}

/// 채권 호가 조회 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondQuote {
    /// 채권일련번호
    #[serde(rename = "BOND_SRNO")]
    pub bond_srno: Option<String>,
    /// 채권명
    #[serde(rename = "BOND_NM")]
    pub bond_nm: Option<String>,
    /// 매도호가1
    #[serde(rename = "ASKP1")]
    pub askp1: Option<String>,
    /// 매수호가1
    #[serde(rename = "BIDP1")]
    pub bidp1: Option<String>,
    /// 매도호가수량1
    #[serde(rename = "ASKP_RSQN1")]
    pub askp_rsqn1: Option<String>,
    /// 매수호가수량1
    #[serde(rename = "BIDP_RSQN1")]
    pub bidp_rsqn1: Option<String>,
    /// 매도호가2
    #[serde(rename = "ASKP2")]
    pub askp2: Option<String>,
    /// 매수호가2
    #[serde(rename = "BIDP2")]
    pub bidp2: Option<String>,
    /// 매도호가수량2
    #[serde(rename = "ASKP_RSQN2")]
    pub askp_rsqn2: Option<String>,
    /// 매수호가수량2
    #[serde(rename = "BIDP_RSQN2")]
    pub bidp_rsqn2: Option<String>,
    /// 매도호가3
    #[serde(rename = "ASKP3")]
    pub askp3: Option<String>,
    /// 매수호가3
    #[serde(rename = "BIDP3")]
    pub bidp3: Option<String>,
    /// 매도호가수량3
    #[serde(rename = "ASKP_RSQN3")]
    pub askp_rsqn3: Option<String>,
    /// 매수호가수량3
    #[serde(rename = "BIDP_RSQN3")]
    pub bidp_rsqn3: Option<String>,
    /// 매도호가4
    #[serde(rename = "ASKP4")]
    pub askp4: Option<String>,
    /// 매수호가4
    #[serde(rename = "BIDP4")]
    pub bidp4: Option<String>,
    /// 매도호가수량4
    #[serde(rename = "ASKP_RSQN4")]
    pub askp_rsqn4: Option<String>,
    /// 매수호가수량4
    #[serde(rename = "BIDP_RSQN4")]
    pub bidp_rsqn4: Option<String>,
    /// 매도호가5
    #[serde(rename = "ASKP5")]
    pub askp5: Option<String>,
    /// 매수호가5
    #[serde(rename = "BIDP5")]
    pub bidp5: Option<String>,
    /// 매도호가수량5
    #[serde(rename = "ASKP_RSQN5")]
    pub askp_rsqn5: Option<String>,
    /// 매수호가수량5
    #[serde(rename = "BIDP_RSQN5")]
    pub bidp_rsqn5: Option<String>,
}

/// 채권 현재가 조회 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondPrice {
    /// 채권일련번호
    #[serde(rename = "BOND_SRNO")]
    pub bond_srno: Option<String>,
    /// 채권명
    #[serde(rename = "BOND_NM")]
    pub bond_nm: Option<String>,
    /// 현재가
    #[serde(rename = "STCK_PRPR")]
    pub stck_prpr: Option<String>,
    /// 전일대비
    #[serde(rename = "PRDY_VRSS")]
    pub prdy_vrss: Option<String>,
    /// 전일대비부호
    #[serde(rename = "PRDY_VRSS_SIGN")]
    pub prdy_vrss_sign: Option<String>,
    /// 등락율
    #[serde(rename = "PRDY_CTRT")]
    pub prdy_ctrt: Option<String>,
    /// 누적거래량
    #[serde(rename = "ACML_VOL")]
    pub acml_vol: Option<String>,
    /// 누적거래대금
    #[serde(rename = "ACML_TR_PBMN")]
    pub acml_tr_pbmn: Option<String>,
    /// 시가
    #[serde(rename = "STCK_OPRC")]
    pub stck_oprc: Option<String>,
    /// 고가
    #[serde(rename = "STCK_HGPR")]
    pub stck_hgpr: Option<String>,
    /// 저가
    #[serde(rename = "STCK_LWPR")]
    pub stck_lwpr: Option<String>,
    /// 만기일자
    #[serde(rename = "EXPR_DT")]
    pub expr_dt: Option<String>,
    /// 표면이율
    #[serde(rename = "SRFC_INRT")]
    pub srfc_inrt: Option<String>,
}

// ============================================================================
// Bond API Implementation
// ============================================================================

impl KisClient {
    /// 장내채권 매수
    ///
    /// # Arguments
    /// * `bond_srno` - 채권일련번호
    /// * `qty` - 주문수량
    /// * `price` - 주문단가
    ///
    /// # Returns
    /// * `KisResult<BondOrderResponse>` - 주문 결과
    pub fn bond_buy(&self, bond_srno: &str, qty: u32, price: &str) -> KisResult<BondOrderResponse> {
        let request = BondBuyRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            bond_srno: bond_srno.to_string(),
            ord_qty: qty.to_string(),
            ord_pric: price.to_string(),
        };

        let tr_id = if self.config.is_paper {
            "VTCB1101U"
        } else {
            "TTCB1101U"
        };

        let response = self.post("/uapi/domestic-bond/v1/trading/buy", tr_id, &request)?;

        let api_response: KisApiResponse<BondOrderResponse> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("No output in response".to_string()))
    }

    /// 장내채권 매도
    ///
    /// # Arguments
    /// * `bond_srno` - 채권일련번호
    /// * `qty` - 주문수량
    /// * `price` - 주문단가
    ///
    /// # Returns
    /// * `KisResult<BondOrderResponse>` - 주문 결과
    pub fn bond_sell(
        &self,
        bond_srno: &str,
        qty: u32,
        price: &str,
    ) -> KisResult<BondOrderResponse> {
        let request = BondSellRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            bond_srno: bond_srno.to_string(),
            ord_qty: qty.to_string(),
            ord_pric: price.to_string(),
        };

        let tr_id = if self.config.is_paper {
            "VTCB1201U"
        } else {
            "TTCB1201U"
        };

        let response = self.post("/uapi/domestic-bond/v1/trading/sell", tr_id, &request)?;

        let api_response: KisApiResponse<BondOrderResponse> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("No output in response".to_string()))
    }

    /// 장내채권 주문 정정
    ///
    /// # Arguments
    /// * `krx_fwdg_ord_orgno` - 한국거래소전송주문조직번호
    /// * `orgn_odno` - 원주문번호
    /// * `qty` - 정정수량
    /// * `price` - 정정단가
    ///
    /// # Returns
    /// * `KisResult<BondOrderResponse>` - 정정 결과
    pub fn bond_revise_order(
        &self,
        krx_fwdg_ord_orgno: &str,
        orgn_odno: &str,
        qty: u32,
        price: &str,
    ) -> KisResult<BondOrderResponse> {
        let request = BondOrderReviseRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            krx_fwdg_ord_orgno: krx_fwdg_ord_orgno.to_string(),
            orgn_odno: orgn_odno.to_string(),
            ord_dvsn: "01".to_string(), // 01: 정정
            rvse_qty: Some(qty.to_string()),
            rvse_pric: Some(price.to_string()),
        };

        let tr_id = if self.config.is_paper {
            "VTCB1301U"
        } else {
            "TTCB1301U"
        };

        let response = self.post(
            "/uapi/domestic-bond/v1/trading/order-rvsecncl",
            tr_id,
            &request,
        )?;

        let api_response: KisApiResponse<BondOrderResponse> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("No output in response".to_string()))
    }

    /// 장내채권 주문 취소
    ///
    /// # Arguments
    /// * `krx_fwdg_ord_orgno` - 한국거래소전송주문조직번호
    /// * `orgn_odno` - 원주문번호
    ///
    /// # Returns
    /// * `KisResult<BondOrderResponse>` - 취소 결과
    pub fn bond_cancel_order(
        &self,
        krx_fwdg_ord_orgno: &str,
        orgn_odno: &str,
    ) -> KisResult<BondOrderResponse> {
        let request = BondOrderReviseRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            krx_fwdg_ord_orgno: krx_fwdg_ord_orgno.to_string(),
            orgn_odno: orgn_odno.to_string(),
            ord_dvsn: "02".to_string(), // 02: 취소
            rvse_qty: Some("0".to_string()),
            rvse_pric: None,
        };

        let tr_id = if self.config.is_paper {
            "VTCB1301U"
        } else {
            "TTCB1301U"
        };

        let response = self.post(
            "/uapi/domestic-bond/v1/trading/order-rvsecncl",
            tr_id,
            &request,
        )?;

        let api_response: KisApiResponse<BondOrderResponse> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("No output in response".to_string()))
    }

    /// 장내채권 잔고 조회
    ///
    /// # Returns
    /// * `KisResult<Vec<BondBalance>>` - 채권 잔고 목록
    pub fn bond_get_balance(&self) -> KisResult<Vec<BondBalance>> {
        let query = format!(
            "CANO={}&ACNT_PRDT_CD={}&AFHR_FLPR_YN=N&OFL_YN=N&INQR_DVSN=01&UNPR_DVSN=01&FUND_STTL_ICLD_YN=N&FNCG_AMT_AUTO_RDPT_YN=N&PRCS_DVSN=00&CTX_AREA_FK100=&CTX_AREA_NK100=",
            self.cano(),
            self.acnt_prdt_cd()
        );

        let tr_id = if self.config.is_paper {
            "VTCB8001R"
        } else {
            "CTCB8001R"
        };

        let response = self.get(
            "/uapi/domestic-bond/v1/trading/inquire-balance",
            tr_id,
            Some(&query),
        )?;

        let api_response: KisApiResponse<BondBalance> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        Ok(api_response.output1.unwrap_or_default())
    }

    /// 장내채권 호가 조회
    ///
    /// # Arguments
    /// * `bond_srno` - 채권일련번호
    ///
    /// # Returns
    /// * `KisResult<BondQuote>` - 채권 호가 정보
    pub fn bond_get_quote(&self, bond_srno: &str) -> KisResult<BondQuote> {
        let query = format!("BOND_SRNO={}", bond_srno);

        let tr_id = if self.config.is_paper {
            "VTCB3001R"
        } else {
            "CTCB3001R"
        };

        let response = self.get(
            "/uapi/domestic-bond/v1/quotations/inquire-asking-price",
            tr_id,
            Some(&query),
        )?;

        let api_response: KisApiResponse<BondQuote> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("No output in response".to_string()))
    }

    /// 장내채권 현재가 조회
    ///
    /// # Arguments
    /// * `bond_srno` - 채권일련번호
    ///
    /// # Returns
    /// * `KisResult<BondPrice>` - 채권 현재가 정보
    pub fn bond_get_price(&self, bond_srno: &str) -> KisResult<BondPrice> {
        let query = format!("BOND_SRNO={}", bond_srno);

        let tr_id = if self.config.is_paper {
            "VTCB3002R"
        } else {
            "CTCB3002R"
        };

        let response = self.get(
            "/uapi/domestic-bond/v1/quotations/inquire-price",
            tr_id,
            Some(&query),
        )?;

        let api_response: KisApiResponse<BondPrice> = response
            .json()
            .map_err(|e| KisError::Parse(format!("Failed to parse response: {}", e)))?;

        if !api_response.is_success() {
            return Err(KisError::Api {
                code: api_response.msg_cd,
                message: api_response.msg1,
            });
        }

        api_response
            .output
            .ok_or_else(|| KisError::Parse("No output in response".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_request_serialization() {
        let request = BondBuyRequest {
            cano: "12345678".to_string(),
            acnt_prdt_cd: "01".to_string(),
            bond_srno: "KR1234567890".to_string(),
            ord_qty: "10".to_string(),
            ord_pric: "100000".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("CANO"));
        assert!(json.contains("BOND_SRNO"));
    }
}

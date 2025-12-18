//! KIS Overseas Stock API
//!
//! 해외주식 주문, 잔고조회, 현재가 조회 API

use super::{
    types::{Exchange, KisApiResponse, KisError, KisResult, OrderSide},
    KisClient,
};
use serde::{Deserialize, Serialize};

/// 해외주식 주문 요청
#[derive(Debug, Clone, Serialize)]
pub struct OverseasOrderRequest {
    /// 계좌번호 (8자리)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (2자리)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 거래소코드
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 종목코드 (예: AAPL)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (시장가: 0)
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 주문서버구분코드
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 주문구분
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
}

/// 해외주식 주문 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasOrderResponse {
    /// 주문번호
    #[serde(rename = "ORD_NO")]
    pub ord_no: Option<String>,
    /// 주문시각
    #[serde(rename = "ORD_TMD")]
    pub ord_tmd: Option<String>,
    /// KRX주문번호
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: Option<String>,
    /// 조직번호
    #[serde(rename = "ODNO")]
    pub odno: Option<String>,
    /// 주문정정취소가능여부
    #[serde(rename = "ORD_ABLEYN")]
    pub ord_ableyn: Option<String>,
}

/// 해외주식 잔고 항목
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasBalanceItem {
    /// 거래소코드
    #[serde(rename = "ovrs_excg_cd")]
    pub ovrs_excg_cd: Option<String>,
    /// 종목코드
    #[serde(rename = "ovrs_pdno")]
    pub ovrs_pdno: Option<String>,
    /// 종목명
    #[serde(rename = "ovrs_item_name")]
    pub ovrs_item_name: Option<String>,
    /// 보유수량
    #[serde(rename = "ovrs_cblc_qty")]
    pub ovrs_cblc_qty: Option<String>,
    /// 평균매입가격(현지화폐)
    #[serde(rename = "frcr_pchs_amt1")]
    pub frcr_pchs_amt1: Option<String>,
    /// 해외현재가격
    #[serde(rename = "ovrs_now_pric1")]
    pub ovrs_now_pric1: Option<String>,
    /// 평가손익금액(외화)
    #[serde(rename = "frcr_evlu_pfls_amt")]
    pub frcr_evlu_pfls_amt: Option<String>,
    /// 평가손익율
    #[serde(rename = "evlu_pfls_rt")]
    pub evlu_pfls_rt: Option<String>,
    /// 평가금액(외화)
    #[serde(rename = "frcr_evlu_amt2")]
    pub frcr_evlu_amt2: Option<String>,
    /// 통화코드
    #[serde(rename = "tr_crcy_cd")]
    pub tr_crcy_cd: Option<String>,
}

/// 해외주식 잔고조회 응답
pub type OverseasBalanceResponse = Vec<OverseasBalanceItem>;

/// 해외주식 현재가 조회 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasPriceResponse {
    /// 실시간조회종목코드
    #[serde(rename = "rsym")]
    pub rsym: Option<String>,
    /// 영업일자
    #[serde(rename = "zdiv")]
    pub zdiv: Option<String>,
    /// 현재가
    #[serde(rename = "last")]
    pub last: Option<String>,
    /// 전일대비
    #[serde(rename = "diff")]
    pub diff: Option<String>,
    /// 등락율
    #[serde(rename = "rate")]
    pub rate: Option<String>,
    /// 시가
    #[serde(rename = "open")]
    pub open: Option<String>,
    /// 고가
    #[serde(rename = "high")]
    pub high: Option<String>,
    /// 저가
    #[serde(rename = "low")]
    pub low: Option<String>,
    /// 거래량
    #[serde(rename = "tvol")]
    pub tvol: Option<String>,
    /// 거래대금
    #[serde(rename = "tamt")]
    pub tamt: Option<String>,
}

/// 미국 주문구분 타입
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UsOrderType {
    /// 00: 지정가
    Limit,
    /// 31: 장개시전시간외(MOO - Market On Open)
    PreMarketMOO,
    /// 32: 장마감전시간외(LOO - Limit On Open)
    PreMarketLOO,
    /// 34: 장마감후시간외(MOC - Market On Close)
    AfterMarketMOC,
}

impl UsOrderType {
    pub fn code(&self) -> &'static str {
        match self {
            UsOrderType::Limit => "00",
            UsOrderType::PreMarketMOO => "31",
            UsOrderType::PreMarketLOO => "32",
            UsOrderType::AfterMarketMOC => "34",
        }
    }
}

impl KisClient {
    /// 해외주식 주문 TR_ID 생성
    fn get_overseas_order_tr_id(&self, exchange: Exchange, side: OrderSide) -> String {
        let prefix = if self.config.is_paper { "V" } else { "T" };

        match exchange {
            Exchange::NYSE | Exchange::NASDAQ | Exchange::AMEX => {
                // 미국
                match side {
                    OrderSide::Buy => format!("{}TTT1002U", prefix),
                    OrderSide::Sell => format!("{}TTT1006U", prefix),
                }
            }
            Exchange::SEHK => {
                // 홍콩
                match side {
                    OrderSide::Buy => format!("{}TTS1002U", prefix),
                    OrderSide::Sell => format!("{}TTS1001U", prefix),
                }
            }
            Exchange::SHAA => {
                // 상해
                match side {
                    OrderSide::Buy => format!("{}TTS0202U", prefix),
                    OrderSide::Sell => format!("{}TTS1005U", prefix),
                }
            }
            Exchange::SZAA => {
                // 심천
                match side {
                    OrderSide::Buy => format!("{}TTS0305U", prefix),
                    OrderSide::Sell => format!("{}TTS0304U", prefix),
                }
            }
            Exchange::TKSE => {
                // 일본
                match side {
                    OrderSide::Buy => format!("{}TTS0308U", prefix),
                    OrderSide::Sell => format!("{}TTS0307U", prefix),
                }
            }
            Exchange::HASE | Exchange::VNSE => {
                // 베트남
                match side {
                    OrderSide::Buy => format!("{}TTS0311U", prefix),
                    OrderSide::Sell => format!("{}TTS0310U", prefix),
                }
            }
        }
    }

    /// 해외주식 주문
    ///
    /// # Arguments
    /// * `exchange` - 거래소 (NASD/NYSE/AMEX/SEHK/SHAA/SZAA/TKSE/HASE/VNSE)
    /// * `symbol` - 종목코드 (예: AAPL)
    /// * `side` - 매수/매도
    /// * `quantity` - 주문수량
    /// * `price` - 주문단가 (시장가의 경우 0.0)
    /// * `order_type` - 주문구분 (미국 주식만 해당, 기본값 "00" 지정가)
    pub fn overseas_order(
        &mut self,
        exchange: Exchange,
        symbol: &str,
        side: OrderSide,
        quantity: u32,
        price: f64,
        order_type: Option<UsOrderType>,
    ) -> KisResult<OverseasOrderResponse> {
        self.ensure_auth()?;

        let tr_id = self.get_overseas_order_tr_id(exchange, side);

        // 주문구분: 미국의 경우 order_type 사용, 그 외는 "00"
        let ord_dvsn = if exchange.is_us() {
            order_type.unwrap_or(UsOrderType::Limit).code()
        } else {
            "00"
        };

        let request = OverseasOrderRequest {
            cano: self.cano().to_string(),
            acnt_prdt_cd: self.acnt_prdt_cd().to_string(),
            ovrs_excg_cd: exchange.code().to_string(),
            pdno: symbol.to_string(),
            ord_qty: quantity.to_string(),
            ovrs_ord_unpr: if price == 0.0 {
                "0".to_string()
            } else {
                format!("{:.2}", price)
            },
            ord_svr_dvsn_cd: "0".to_string(),
            ord_dvsn: ord_dvsn.to_string(),
        };

        // Hashkey 생성
        let hashkey = self.get_hashkey(&request)?;

        // 헤더에 hashkey 추가
        let mut headers = self.build_headers(&tr_id)?;
        headers.insert("hashkey".to_string(), hashkey);

        // POST 요청
        let response = self.http.post_json(
            "/uapi/overseas-stock/v1/trading/order",
            &request,
            Some(headers),
        );

        if !response.is_success() {
            return Err(KisError::Api {
                code: response.status.to_string(),
                message: response.error.unwrap_or_else(|| response.body.clone()),
            });
        }

        let api_response: KisApiResponse<OverseasOrderResponse> = response
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

    /// 해외주식 잔고조회
    pub fn overseas_balance(&mut self) -> KisResult<OverseasBalanceResponse> {
        self.ensure_auth()?;

        let tr_id = if self.config.is_paper {
            "VTTS3012R"
        } else {
            "TTTS3012R"
        };

        // 쿼리 파라미터 구성
        let query = format!(
            "CANO={}&ACNT_PRDT_CD={}&OVRS_EXCG_CD=&TR_CRCY_CD=&CTX_AREA_FK200=&CTX_AREA_NK200=",
            self.cano(),
            self.acnt_prdt_cd()
        );

        let response = self.get(
            "/uapi/overseas-stock/v1/trading/inquire-balance",
            tr_id,
            Some(&query),
        )?;

        let api_response: KisApiResponse<OverseasBalanceItem> = response
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

    /// 해외주식 현재가 조회
    ///
    /// # Arguments
    /// * `exchange` - 거래소
    /// * `symbol` - 종목코드 (예: AAPL)
    pub fn overseas_price(
        &mut self,
        exchange: Exchange,
        symbol: &str,
    ) -> KisResult<OverseasPriceResponse> {
        self.ensure_auth()?;

        let tr_id = "HHDFS00000300";

        // 쿼리 파라미터 구성
        let query = format!("AUTH=&EXCD={}&SYMB={}", exchange.code(), symbol);

        let response = self.get(
            "/uapi/overseas-stock/v1/quotations/price",
            tr_id,
            Some(&query),
        )?;

        let api_response: KisApiResponse<OverseasPriceResponse> = response
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
    use crate::kis::types::KisConfig;

    #[test]
    fn test_tr_id_generation() {
        // 실전 모드
        let config = KisConfig::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "1234567890".to_string(),
            false,
        );
        let client = KisClient::new(config);

        // 미국 매수
        assert_eq!(
            client.get_overseas_order_tr_id(Exchange::NASDAQ, OrderSide::Buy),
            "TTTT1002U"
        );
        // 미국 매도
        assert_eq!(
            client.get_overseas_order_tr_id(Exchange::NYSE, OrderSide::Sell),
            "TTTT1006U"
        );
        // 홍콩 매수
        assert_eq!(
            client.get_overseas_order_tr_id(Exchange::SEHK, OrderSide::Buy),
            "TTTS1002U"
        );

        // 모의 모드
        let config_paper = KisConfig::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "1234567890".to_string(),
            true,
        );
        let client_paper = KisClient::new(config_paper);

        // 미국 매수 (모의)
        assert_eq!(
            client_paper.get_overseas_order_tr_id(Exchange::NASDAQ, OrderSide::Buy),
            "VTTT1002U"
        );
    }

    #[test]
    fn test_us_order_type_codes() {
        assert_eq!(UsOrderType::Limit.code(), "00");
        assert_eq!(UsOrderType::PreMarketMOO.code(), "31");
        assert_eq!(UsOrderType::PreMarketLOO.code(), "32");
        assert_eq!(UsOrderType::AfterMarketMOC.code(), "34");
    }
}

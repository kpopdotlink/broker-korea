# broker-korea 플러그인 구현 태스크

## 개요

한국투자증권(KIS) OpenAPI를 연동하여 국내/해외 주식, 선물옵션, 채권 거래를 지원하는 완벽한 브로커 플러그인.

### 참고 자료
- [KIS Developers](https://apiportal.koreainvestment.com)
- [공식 GitHub](https://github.com/koreainvestment/open-trading-api)

### 환경 구분
| 환경 | Base URL | TR_ID 접두사 |
|------|----------|-------------|
| 실전투자 | `https://openapi.koreainvestment.com:9443` | `T` |
| 모의투자 | `https://openapivts.koreainvestment.com:29443` | `V` |

---

## TASK 1: 인증 모듈 (OAuth, Hashkey)

### 1.1 접근토큰 발급

**Endpoint:** `POST /oauth2/tokenP`

**Request:**
```json
{
  "grant_type": "client_credentials",
  "appkey": "발급받은 앱키",
  "appsecret": "발급받은 앱시크릿"
}
```

**Response:**
```json
{
  "access_token": "eyJ...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "access_token_token_expired": "2024-01-01 12:00:00"
}
```

**특이사항:**
- 토큰 유효기간: 24시간
- 6시간 이내 재발급 요청시 기존 토큰 반환
- 1분당 1회 발급 제한

### 1.2 Hashkey 발급

POST 요청(주문 등) 시 필요한 보안키

**Endpoint:** `POST /uapi/hashkey`

**Headers:**
```
appkey: 발급받은 앱키
appsecret: 발급받은 앱시크릿
```

**Request:** 주문 요청 body 그대로

**Response:**
```json
{
  "BODY": {
    "HASH": "해시값"
  }
}
```

### 1.3 WebSocket 접속키 발급

실시간 시세용

**Endpoint:** `POST /oauth2/Approval`

**Request:**
```json
{
  "grant_type": "client_credentials",
  "appkey": "앱키",
  "secretkey": "앱시크릿"
}
```

---

## TASK 2: 국내주식 API

### 2.1 주문

#### 현금 매수/매도

**Endpoint:** `POST /uapi/domestic-stock/v1/trading/order-cash`

**TR_ID:**
| 구분 | 실전 | 모의 |
|-----|-----|-----|
| 매수 | TTTC0802U | VTTC0802U |
| 매도 | TTTC0801U | VTTC0801U |

**Headers:**
```
authorization: Bearer {token}
appkey: {앱키}
appsecret: {앱시크릿}
tr_id: TTTC0802U
custtype: P
hashkey: {해시키}
```

**Request Body:**
```json
{
  "CANO": "계좌번호 앞 8자리",
  "ACNT_PRDT_CD": "계좌번호 뒤 2자리",
  "PDNO": "종목코드 (예: 005930)",
  "ORD_DVSN": "주문구분 (00:지정가, 01:시장가)",
  "ORD_QTY": "주문수량",
  "ORD_UNPR": "주문단가 (시장가:0)"
}
```

**주문구분 코드:**
| 코드 | 설명 |
|-----|-----|
| 00 | 지정가 |
| 01 | 시장가 |
| 02 | 조건부지정가 |
| 03 | 최유리지정가 |
| 04 | 최우선지정가 |
| 05 | 장전시간외 |
| 06 | 장후시간외 |

**Response:**
```json
{
  "rt_cd": "0",
  "msg_cd": "APBK0013",
  "msg1": "주문완료",
  "output": {
    "KRX_FWDG_ORD_ORGNO": "주문조직번호",
    "ODNO": "주문번호",
    "ORD_TMD": "주문시각"
  }
}
```

#### 정정/취소

**Endpoint:** `POST /uapi/domestic-stock/v1/trading/order-rvsecncl`

**TR_ID:** TTTC0803U (정정), TTTC0803U (취소)

### 2.2 잔고조회

**Endpoint:** `GET /uapi/domestic-stock/v1/trading/inquire-balance`

**TR_ID:** TTTC8434R (실전) / VTTC8434R (모의)

**Query Parameters:**
```
CANO=계좌번호8자리
ACNT_PRDT_CD=상품코드2자리
AFHR_FLPR_YN=N
OFL_YN=
INQR_DVSN=02
UNPR_DVSN=01
FUND_STTL_ICLD_YN=N
FNCG_AMT_AUTO_RDPT_YN=N
PRCS_DVSN=00
CTX_AREA_FK100=
CTX_AREA_NK100=
```

**Response output1 (보유종목):**
```json
{
  "pdno": "종목코드",
  "prdt_name": "종목명",
  "hldg_qty": "보유수량",
  "pchs_avg_pric": "매입평균가",
  "prpr": "현재가",
  "evlu_pfls_amt": "평가손익",
  "evlu_pfls_rt": "수익률"
}
```

**Response output2 (계좌요약):**
```json
{
  "dnca_tot_amt": "예수금총액",
  "tot_evlu_amt": "총평가금액",
  "pchs_amt_smtl_amt": "매입금액합계",
  "evlu_amt_smtl_amt": "평가금액합계"
}
```

### 2.3 현재가 조회

**Endpoint:** `GET /uapi/domestic-stock/v1/quotations/inquire-price`

**TR_ID:** FHKST01010100

**Query:** `FID_COND_MRKT_DIV_CODE=J&FID_INPUT_ISCD=005930`

---

## TASK 3: 해외주식 API

### 3.1 주문

**Endpoint:** `POST /uapi/overseas-stock/v1/trading/order`

**TR_ID (거래소별):**

| 거래소 | 매수(실전) | 매수(모의) | 매도(실전) | 매도(모의) |
|-------|----------|----------|----------|----------|
| 미국(NASD/NYSE/AMEX) | TTTT1002U | VTTT1002U | TTTT1006U | VTTT1006U |
| 홍콩(SEHK) | TTTS1002U | VTTS1002U | TTTS1001U | VTTS1001U |
| 상해(SHAA) | TTTS0202U | VTTS0202U | TTTS1005U | VTTS1005U |
| 심천(SZAA) | TTTS0305U | VTTS0305U | TTTS0304U | VTTS0304U |
| 일본(TKSE) | TTTS0308U | VTTS0308U | TTTS0307U | VTTS0307U |
| 베트남(HASE/VNSE) | TTTS0311U | VTTS0311U | TTTS0310U | VTTS0310U |

**Request Body:**
```json
{
  "CANO": "계좌번호8자리",
  "ACNT_PRDT_CD": "상품코드2자리",
  "OVRS_EXCG_CD": "거래소코드 (NASD/NYSE/AMEX/SEHK/...)",
  "PDNO": "종목코드 (예: AAPL)",
  "ORD_QTY": "주문수량",
  "OVRS_ORD_UNPR": "주문단가 (시장가:0)",
  "ORD_SVR_DVSN_CD": "0",
  "ORD_DVSN": "00"
}
```

**주문구분(미국):**
| 코드 | 설명 |
|-----|-----|
| 00 | 지정가 |
| 31 | 장개시전시간외 (MOO) |
| 32 | 장마감전시간외 (LOO) |
| 34 | 장마감후시간외 (MOC) |

### 3.2 잔고조회

**Endpoint:** `GET /uapi/overseas-stock/v1/trading/inquire-balance`

**TR_ID:** TTTS3012R (실전) / VTTS3012R (모의)

### 3.3 현재가 조회

**Endpoint:** `GET /uapi/overseas-stock/v1/quotations/price`

**TR_ID:** HHDFS00000300

---

## TASK 4: 국내선물옵션 API

### 4.1 주문

**Endpoint:** `POST /uapi/domestic-futureoption/v1/trading/order`

**TR_ID:**
| 구분 | 실전 | 모의 |
|-----|-----|-----|
| 신규매수 | TTTO0101U | VTTO0101U |
| 신규매도 | TTTO0102U | VTTO0102U |
| 청산매수 | TTTO0103U | VTTO0103U |
| 청산매도 | TTTO0104U | VTTO0104U |

**Request Body:**
```json
{
  "CANO": "계좌번호8자리",
  "ACNT_PRDT_CD": "03",
  "PDNO": "종목코드 (예: 101S3000)",
  "SLL_BUY_DVSN_CD": "매도수구분 (01:매도, 02:매수)",
  "ORD_QTY": "주문수량",
  "UNIT_PRICE": "주문단가",
  "NMPR_TYPE_CD": "호가유형 (1:지정가, 2:시장가)"
}
```

### 4.2 잔고조회

**Endpoint:** `GET /uapi/domestic-futureoption/v1/trading/inquire-balance`

**TR_ID:** TTTO5201R (실전)

### 4.3 증거금 조회

**Endpoint:** `GET /uapi/domestic-futureoption/v1/trading/inquire-deposit`

---

## TASK 5: 해외선물옵션 API

### 5.1 주문

**Endpoint:** `POST /uapi/overseas-futureoption/v1/trading/order`

**TR_ID:**
| 구분 | 실전 |
|-----|-----|
| 신규매수 | OTFM3001U |
| 신규매도 | OTFM3002U |
| 청산매수 | OTFM3003U |
| 청산매도 | OTFM3004U |

**Request Body:**
```json
{
  "CANO": "계좌번호8자리",
  "ACNT_PRDT_CD": "08",
  "OVRS_FUTR_FX_PDNO": "종목코드",
  "SLL_BUY_DVSN_CD": "01:매도, 02:매수",
  "PRIC_DVSN_CD": "1:지정가, 2:시장가",
  "ORD_QTY": "주문수량",
  "FUOP_LIMT_PRIC": "지정가격"
}
```

### 5.2 잔고조회

**Endpoint:** `GET /uapi/overseas-futureoption/v1/trading/inquire-unpd`

**TR_ID:** OTFM3304R

---

## TASK 6: 장내채권 API

### 6.1 매수

**Endpoint:** `POST /uapi/domestic-bond/v1/trading/buy`

**TR_ID:** TTCB1101U

**Request Body:**
```json
{
  "CANO": "계좌번호8자리",
  "ACNT_PRDT_CD": "상품코드2자리",
  "BOND_SRNO": "채권일련번호",
  "ORD_QTY": "주문수량",
  "ORD_PRIC": "주문단가"
}
```

### 6.2 매도

**Endpoint:** `POST /uapi/domestic-bond/v1/trading/sell`

**TR_ID:** TTCB1201U

### 6.3 잔고조회

**Endpoint:** `GET /uapi/domestic-bond/v1/trading/inquire-balance`

**TR_ID:** CTCB8001R

---

## TASK 7: 통합 및 테스트

### 7.1 플러그인 인터페이스 구현

기존 플러그인 인터페이스에 맞게 통합:
- `get_accounts()` - 계좌 목록 조회
- `get_positions()` - 보유 포지션 조회
- `submit_order()` - 주문 제출
- `initialize()` - 초기화 및 인증

### 7.2 통합 테스트

- 인증 테스트 (모의투자)
- 각 자산군별 주문 테스트
- 잔고 조회 테스트
- 에러 핸들링 테스트

### 7.3 문서화

- README.md 업데이트
- 설정 가이드 작성
- 예제 코드 추가

---

## 공통 헤더

모든 API 요청에 필요한 공통 헤더:

```
Content-Type: application/json; charset=utf-8
authorization: Bearer {access_token}
appkey: {앱키}
appsecret: {앱시크릿}
tr_id: {거래ID}
custtype: P
```

주문 POST 요청 시 추가:
```
hashkey: {해시키}
```

---

## 에러 코드

| 코드 | 설명 |
|-----|-----|
| 0 | 성공 |
| -1 | 실패 |

rt_cd가 "0"이 아니면 msg1에서 에러 메시지 확인.

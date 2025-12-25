# broker-korea 플러그인 구현 태스크

## 개요

한국투자증권(KIS) OpenAPI를 연동하여 국내/해외 주식, 선물옵션, 채권 거래를 지원하는 브로커 플러그인.

### 참고 자료
- [KIS Developers](https://apiportal.koreainvestment.com)
- [공식 GitHub](https://github.com/koreainvestment/open-trading-api)

### 환경 구분
| 환경 | Base URL | TR_ID 접두사 |
|------|----------|-------------|
| 실전투자 | `https://openapi.koreainvestment.com:9443` | `T` |
| 모의투자 | `https://openapivts.koreainvestment.com:29443` | `V` |

---

## 구현 현황

| TASK | 설명 | 상태 |
|------|------|------|
| TASK 1 | 인증 모듈 (OAuth, Hashkey) | ✅ 완료 |
| TASK 2 | 국내주식 API | ✅ 완료 |
| TASK 3 | 해외주식 API | ✅ 완료 |
| TASK 4 | 국내선물옵션 API | ✅ 완료 |
| TASK 5 | 해외선물옵션 API | ✅ 완료 |
| TASK 6 | 장내채권 API | ✅ 완료 |
| TASK 7 | 플러그인 인터페이스 | ✅ 완료 |
| TASK 8 | 호스트 함수 바인딩 | ✅ 완료 |
| TASK 9 | 통합 테스트 | ✅ 완료 |

---

## TASK 1: 인증 모듈 (OAuth, Hashkey) ✅

### 구현 파일
- `src/kis/auth.rs`

### 1.1 접근토큰 발급 ✅

**Endpoint:** `POST /oauth2/tokenP`

**구현:**
```rust
// kis/auth.rs
impl KisClient {
    pub fn ensure_auth(&mut self) -> KisResult<()>
    pub fn get_token(&mut self) -> KisResult<String>
}
```

**특이사항:**
- 토큰 유효기간: 24시간
- 만료 5분 전 자동 갱신
- 토큰 캐싱 구현

### 1.2 Hashkey 발급 ✅

**Endpoint:** `POST /uapi/hashkey`

**구현:**
```rust
impl KisClient {
    pub fn get_hashkey<T: Serialize>(&self, body: &T) -> KisResult<String>
}
```

### 1.3 WebSocket 접속키 발급 ✅

**Endpoint:** `POST /oauth2/Approval`

**구현:**
```rust
pub fn get_websocket_key(http: &HttpClient, config: &KisConfig) -> KisResult<String>
```

---

## TASK 2: 국내주식 API ✅

### 구현 파일
- `src/kis/domestic_stock.rs`

### 2.1 주문 ✅

#### 현금 매수/매도

**구현:**
```rust
impl KisClient {
    pub fn domestic_buy(&mut self, symbol: &str, qty: u32, price: u32, order_type: OrderType) -> KisResult<CashOrderResponse>
    pub fn domestic_sell(&mut self, symbol: &str, qty: u32, price: u32, order_type: OrderType) -> KisResult<CashOrderResponse>
}
```

**TR_ID:**
| 구분 | 실전 | 모의 |
|-----|-----|-----|
| 매수 | TTTC0802U | VTTC0802U |
| 매도 | TTTC0801U | VTTC0801U |

#### 정정/취소 ✅

**구현:**
```rust
impl KisClient {
    pub fn domestic_revise_order(&mut self, org_order_no: &str, qty: u32, price: u32) -> KisResult<OrderReviseResponse>
    pub fn domestic_cancel_order(&mut self, org_order_no: &str, qty: u32) -> KisResult<OrderReviseResponse>
}
```

### 2.2 잔고조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn domestic_balance(&mut self) -> KisResult<(Vec<BalanceItem>, BalanceSummary)>
}
```

**TR_ID:** TTTC8434R (실전) / VTTC8434R (모의)

### 2.3 현재가 조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn domestic_current_price(&mut self, symbol: &str) -> KisResult<CurrentPriceResponse>
}
```

**TR_ID:** FHKST01010100

---

## TASK 3: 해외주식 API ✅

### 구현 파일
- `src/kis/overseas_stock.rs`

### 3.1 주문 ✅

**구현:**
```rust
impl KisClient {
    pub fn overseas_order(&mut self, exchange: Exchange, symbol: &str, side: OrderSide, qty: u32, price: f64, order_type: UsOrderType) -> KisResult<OverseasOrderResponse>
}
```

**지원 거래소:**
- NYSE, NASDAQ, AMEX (미국)
- SEHK (홍콩)
- SHAA (상해)
- SZAA (심천)
- TKSE (일본)
- HASE, VNSE (베트남)

### 3.2 잔고조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn overseas_balance(&mut self) -> KisResult<OverseasBalanceResponse>
}
```

### 3.3 현재가 조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn overseas_price(&mut self, exchange: Exchange, symbol: &str) -> KisResult<OverseasPriceResponse>
}
```

---

## TASK 4: 국내선물옵션 API ✅

### 구현 파일
- `src/kis/domestic_future.rs`

### 4.1 주문 ✅

**구현:**
```rust
impl KisClient {
    pub fn future_order(&mut self, action: FutureOrderAction, symbol: &str, quantity: u32, price: &str, price_type: FuturePriceType) -> KisResult<FutureOrderResponse>
    pub fn future_new_buy(...)
    pub fn future_new_sell(...)
    pub fn future_liquidate_buy(...)
    pub fn future_liquidate_sell(...)
    pub fn future_modify_order(...)
    pub fn future_cancel_order(...)
}
```

**TR_ID:**
| 구분 | 실전 | 모의 |
|-----|-----|-----|
| 신규매수 | TTTO0101U | VTTO0101U |
| 신규매도 | TTTO0102U | VTTO0102U |
| 청산매수 | TTTO0103U | VTTO0103U |
| 청산매도 | TTTO0104U | VTTO0104U |

### 4.2 잔고조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn future_get_balance(&mut self) -> KisResult<Vec<FutureBalance>>
}
```

### 4.3 증거금 조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn future_get_deposit(&mut self) -> KisResult<FutureDeposit>
}
```

### 4.4 체결내역 조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn future_get_executions(&mut self, date: Option<&str>) -> KisResult<Vec<FutureExecution>>
}
```

---

## TASK 5: 해외선물옵션 API ✅

### 구현 파일
- `src/kis/overseas_future.rs`

### 5.1 주문 ✅

**구현:**
```rust
impl KisClient {
    pub fn overseas_future_order(&self, symbol: &str, side: OrderSide, position_type: PositionType, price_type: PriceType, quantity: u32, limit_price: f64) -> KisResult<OrderResult>
    pub fn overseas_future_cancel(&self, order_no: &str) -> KisResult<OrderResult>
    pub fn overseas_future_modify(&self, order_no: &str, quantity: Option<u32>, limit_price: Option<f64>) -> KisResult<OrderResult>
}
```

**TR_ID:**
| 구분 | 실전 |
|-----|-----|
| 신규매수 | OTFM3001U |
| 신규매도 | OTFM3002U |
| 청산매수 | OTFM3003U |
| 청산매도 | OTFM3004U |

### 5.2 잔고조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn overseas_future_inquire_unsettled(&self) -> KisResult<Vec<UnsettledPosition>>
}
```

### 5.3 증거금 조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn overseas_future_inquire_deposit(&self) -> KisResult<DepositInfo>
}
```

### 5.4 체결내역 조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn overseas_future_inquire_execution(&self, start_date: &str, end_date: &str) -> KisResult<Vec<ExecutionRecord>>
}
```

---

## TASK 6: 장내채권 API ✅

### 구현 파일
- `src/kis/bond.rs`

### 6.1 매수/매도 ✅

**구현:**
```rust
impl KisClient {
    pub fn bond_buy(&self, bond_srno: &str, qty: u32, price: &str) -> KisResult<BondOrderResponse>
    pub fn bond_sell(&self, bond_srno: &str, qty: u32, price: &str) -> KisResult<BondOrderResponse>
    pub fn bond_revise_order(&self, org_order_no: &str, qty: u32, price: &str) -> KisResult<BondOrderResponse>
    pub fn bond_cancel_order(&self, org_order_no: &str, qty: u32) -> KisResult<BondOrderResponse>
}
```

### 6.2 잔고조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn bond_get_balance(&self) -> KisResult<Vec<BondBalance>>
}
```

### 6.3 호가/현재가 조회 ✅

**구현:**
```rust
impl KisClient {
    pub fn bond_get_quote(&self, bond_srno: &str) -> KisResult<BondQuote>
    pub fn bond_get_price(&self, bond_srno: &str) -> KisResult<BondPrice>
}
```

---

## TASK 7: 플러그인 인터페이스 통합 ✅

### 구현 파일
- `src/lib.rs`

### 7.1 WASM 엔트리포인트 ✅

**구현:**
```rust
#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32

#[no_mangle]
pub extern "C" fn initialize(ptr: i32, len: i32) -> u64

#[no_mangle]
pub extern "C" fn get_accounts(ptr: i32, len: i32) -> u64

#[no_mangle]
pub extern "C" fn get_positions(ptr: i32, len: i32) -> u64

#[no_mangle]
pub extern "C" fn submit_order(ptr: i32, len: i32) -> u64
```

### 7.2 KisClient 브릿지 연결 ✅

- `initialize()`: KisClient 생성 및 설정 검증
- `get_accounts()`: `domestic_balance()` → `AccountSummary`
- `get_positions()`: `domestic_balance()` → `Vec<Position>`
- `submit_order()`: `domestic_buy/sell()` → `Order`

---

## TASK 8: 호스트 함수 바인딩 ✅

### 구현 완료
- `plugin_api/src/http.rs` ✅ 타입 정의 완료
- `plugin_runtime/src/host_functions.rs` ✅ HttpClient 구현 완료
- `plugin_runtime/src/sandbox.rs` ✅ wasmtime linker에 `http_request` 함수 바인딩 (line 97-135)
- `plugin_runtime/src/loader.rs` ✅ 플러그인 로드 시 host_functions 연결 (line 238-264)
- 보안 정책 ✅ allowed_hosts deny-by-default 정책 적용

### 구현 상세
```rust
// sandbox.rs - linker에 http_request 바인딩
linker.func_wrap_async("env", "http_request", |caller, (ptr, len)| {
    // 1. 네트워크 권한 검사
    // 2. 게스트 메모리에서 요청 읽기
    // 3. HTTP 요청 실행 (allowed_hosts 검증)
    // 4. 응답을 게스트 메모리에 쓰기
})

// loader.rs - 플러그인 로드 시 HTTP client 생성
let http_client = if has_network_permission {
    Some(HttpClient::new().with_allowed_hosts(manifest.allowed_hosts))
} else {
    None
};
```

---

## TASK 9: 통합 테스트 ✅

### 구현 완료
테스트는 CI에서 자동 실행됩니다 (`.github/workflows/ci.yml`).

### 테스트 파일
- `desktop/crates/plugin_runtime/tests/broker_tests.rs` - 브로커 플러그인 통합 테스트

### 테스트 항목
- [x] 플러그인 빌드 및 로드 테스트 (CI)
- [x] 매니페스트 검증 테스트
- [x] WASM 모듈 인스턴스화 테스트
- [x] 에러 핸들링 테스트

### 수동 테스트 (API 키 필요)
```bash
# 환경 변수 설정
export KIS_APP_KEY="..."
export KIS_APP_SECRET="..."
export KIS_ACCOUNT_NO="..." # 10자리

# 모의투자 테스트
cargo test --features integration_test
```

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

---

## 변경 이력

| 날짜 | 버전 | 내용 |
|------|------|------|
| 2024-12-18 | 0.1.0 | KIS API 클라이언트 구현 완료 |
| 2024-12-18 | 0.1.1 | lib.rs 브릿지 연결 완료, WASM 빌드 성공 |

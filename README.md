# broker-korea

한국투자증권(KIS) OpenAPI 연동 플러그인 for KL Investment.

## 개요

KIS OpenAPI를 통해 실전/모의 투자 환경에서 주식, 선물옵션, 채권 거래를 지원하는 WASM 플러그인입니다.

### 지원 자산

| 자산군 | 구현 상태 | 설명 |
|--------|----------|------|
| 국내주식 | ✅ 완료 | 매수/매도/정정/취소, 잔고/현재가 조회 |
| 해외주식 | ✅ 구현됨 | NYSE, NASDAQ, AMEX, 홍콩, 상해, 심천, 일본, 베트남 |
| 국내선물옵션 | ✅ 구현됨 | 신규/청산 매수매도, 잔고/증거금 조회 |
| 해외선물옵션 | ✅ 구현됨 | 신규/청산 매수매도, 미결제/증거금 조회 |
| 장내채권 | ✅ 구현됨 | 매수/매도/정정/취소, 잔고/호가/현재가 조회 |

### 플러그인 인터페이스

| 함수 | 연동 API | 상태 |
|------|----------|------|
| `initialize()` | OAuth 토큰 발급 준비 | ✅ |
| `get_accounts()` | 국내주식 잔고조회 | ✅ |
| `get_positions()` | 국내주식 잔고조회 | ✅ |
| `submit_order()` | 국내주식 현금매수/매도 | ✅ |

## 설정

### 1. KIS Developers 앱 등록

1. [KIS Developers](https://apiportal.koreainvestment.com)에서 앱 등록
2. 모의투자 또는 실전투자 앱키 발급

### 2. Secrets 설정

KL Investment 앱에서 다음 secrets 설정:

```
broker-korea:app_key      - 발급받은 앱키
broker-korea:app_secret   - 발급받은 앱시크릿
broker-korea:account_no   - 계좌번호 10자리 (CANO 8자리 + ACNT_PRDT_CD 2자리)
broker-korea:is_paper     - 모의투자 여부 (true/false)
```

### 3. 빌드

```bash
# WASM 타겟 추가 (최초 1회)
rustup target add wasm32-wasip1

# 빌드
cargo build --target wasm32-wasip1 --release

# 결과물: target/wasm32-wasip1/release/broker_korea.wasm
```

## 아키텍처

```
broker-korea/
├── src/
│   ├── lib.rs              # WASM 진입점, 플러그인 인터페이스
│   ├── http.rs             # HTTP 호스트 함수 래퍼
│   └── kis/
│       ├── mod.rs          # KisClient 코어
│       ├── types.rs        # 공통 타입, 에러 정의
│       ├── auth.rs         # OAuth 토큰, Hashkey 발급
│       ├── domestic_stock.rs   # 국내주식 API
│       ├── overseas_stock.rs   # 해외주식 API
│       ├── domestic_future.rs  # 국내선물옵션 API
│       ├── overseas_future.rs  # 해외선물옵션 API
│       └── bond.rs             # 장내채권 API
├── manifest.json           # 플러그인 매니페스트
├── Cargo.toml
└── README.md
```

## API 환경

| 환경 | Base URL | TR_ID 접두사 |
|------|----------|-------------|
| 실전투자 | `https://openapi.koreainvestment.com:9443` | `T` |
| 모의투자 | `https://openapivts.koreainvestment.com:29443` | `V` |

## 데이터 매핑

### 잔고 조회 (get_accounts)

```
KIS API                          → Plugin API
────────────────────────────────────────────────
BalanceSummary.tot_evlu_amt      → AccountBalance.total_equity
BalanceSummary.ord_psbl_cash     → AccountBalance.available_cash
BalanceSummary.dnca_tot_amt      → AccountBalance.buying_power
BalanceItem.pdno                 → Position.symbol_id
BalanceItem.hldg_qty             → Position.quantity
BalanceItem.pchs_avg_pric        → Position.average_price
BalanceItem.prpr                 → Position.current_price
BalanceItem.evlu_pfls_amt        → Position.unrealized_pnl
BalanceItem.evlu_pfls_rt         → Position.unrealized_pnl_percent
```

### 주문 (submit_order)

```
Plugin API                       → KIS API
────────────────────────────────────────────────
OrderRequest.symbol_id           → PDNO (종목코드)
OrderRequest.quantity            → ORD_QTY (주문수량)
OrderRequest.limit_price         → ORD_UNPR (주문단가)
OrderType::Market                → ORD_DVSN = "01"
OrderType::Limit                 → ORD_DVSN = "00"
OrderSide::Buy                   → TR_ID = TTTC0802U
OrderSide::Sell                  → TR_ID = TTTC0801U
```

## 개발

### Git 저장소

이 플러그인은 별도의 private 저장소로 관리됩니다:

```bash
# 저장소 정보
origin: https://github.com/kpopdotlink/broker-korea.git
branch: main

# 작업 흐름
git add -A
git commit -m "your message"
git push origin main
```

### 테스트

```bash
# 유닛 테스트
cargo test

# WASM 빌드 테스트
cargo build --target wasm32-wasip1
```

## 제한사항

1. **호스트 함수 필요**: WASM 플러그인이 HTTP 요청을 하려면 호스트(plugin_runtime)에서 `http_request` 함수를 제공해야 함
2. **허용된 호스트만**: 보안상 `openapi.koreainvestment.com`, `openapivts.koreainvestment.com`만 접근 가능
3. **API 제한**: KIS API rate limit 준수 필요 (토큰 발급: 1분당 1회)

## 참고 자료

- [KIS Developers Portal](https://apiportal.koreainvestment.com)
- [KIS OpenAPI GitHub](https://github.com/koreainvestment/open-trading-api)
- [KL Investment 메인 프로젝트](https://github.com/kpopdotlink/klinvestment)

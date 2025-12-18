# broker-korea

Korean securities broker integration plugin for KL Investment.

## Setup

1. Configure secrets in the main app:
   - `broker-korea:app_key` - API App Key
   - `broker-korea:app_secret` - API App Secret
   - `broker-korea:account_no` - Trading Account Number

2. Build the WASM plugin:
   ```bash
   cargo build --target wasm32-wasip1 --release
   ```

3. Copy the built WASM to the plugins directory or enable via the app.

## Development

This is a private repository. Do not push to the main klinvestment repo.

```bash
# Initialize (first time)
git init
git remote add origin <your-private-repo-url>

# Regular workflow
git add .
git commit -m "your message"
git push origin main
```

## API Implementation

TODO: Implement actual broker API calls in `src/lib.rs`:
- `get_accounts`: Fetch real account info
- `get_positions`: Fetch real positions
- `submit_order`: Submit orders to broker

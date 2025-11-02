# Solana Copy Trading Bot

High-performance copy trading system for the Solana blockchain, written in Rust.

## Description

**Bot running in docker in cargo run mode**

- Mirrors buys/sells from multiple target wallets on Pump.fun.
- Parser throughput: 500k–700k tx/s in benchmarks.
- End-to-end latency: 2–5 ms from Geyser event to transaction submit.
- Slot timing: consistently lands in slot 0–1.
- Network appearance: ~120 ms from receive → on-chain (per Geyser tracking).
- Supports transaction simulation mode for dry-runs and strategy testing.
- Fees: configurable priority fee/tip. Defaults low at $0.10–$0.20 per tx, increase max-fee caps for faster inclusion
  when needed.
- Landing provider: implemented https://0slot.trade

Purpose-built for ultra-low-latency copy execution with multi-wallet targeting and deterministic behavior—no fluff, just
speed and reliability.

## Quickstart

> **Note:** For best performance, use a **paid Geyser stream** and a **low-latency RPC (landing) provider**. Deploy your
> bot **as close as possible** to the Geyser and RPC servers (same region or data center) to minimize network delay.

### Configure environment

Copy an example env file and adjust values as needed:

```bash
cp .env.example .env
```

### Start docker

```bash
docker compose up -d
```

### Run migrations (first time)

```bash
docker compose ps 
# container solana-app must be running
docker exec -it solana-app bash
diesel migration run
exit
```

### Restart the stack to apply:

```bash
docker compose down
docker compose up
```

Now the bot is ready — view logs in the running Docker container to confirm startup.

## Disclaimer

For research & educational purposes only. Not financial advice. Use at your own risk. You are responsible for compliance
with local laws and platform terms.

## License

MIT
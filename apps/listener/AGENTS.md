# Listener Service (`apps/listener`)

## Language and Entrypoint
* **Language**: Rust 1.78
* **Entrypoint**: `src/main.rs`

## Build
```bash
cargo build --release
```

## Run
```bash
cargo run -- --rpc $HELIUS_KEY --kafka $KAFKA_BROKERS
```

## Tests
```bash
cargo test
```

## Key Library References
* `solana-client`: For interacting with the Solana RPC.
* `tokio`: Asynchronous runtime.
* `rdkafka`: For producing messages to Kafka. 
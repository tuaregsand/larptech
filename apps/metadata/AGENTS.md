# Metadata Service (`apps/metadata`)

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
Unit tests mock PDA fetch JSON to avoid network calls.

## Key Library References
* `solana-client`: For interacting with the Solana RPC.
* `tokio`: Asynchronous runtime.
* `rdkafka`: For consuming and producing messages to Kafka.
* `mpl-token-metadata`: For fetching and deserializing Metaplex token metadata. 
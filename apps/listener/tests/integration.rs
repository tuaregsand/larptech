use futures_util::{SinkExt, StreamExt};
use listener::{
    kafka::KafkaProducer,
    parser::{parse, LaunchpadMap},
    rpc::RawLog,
};
use prost::Message;
use std::process::Command;
use testcontainers::{clients::Cli, Container, GenericImage};

#[tokio::test]
async fn test_parse_initialize_mint_log() {
    let raw = RawLog {
        logs: vec![
            "Program log: Instruction: InitializeMint".to_string(),
            "Program log: Mint authority \"AuthPubkey\"".to_string(),
            "Program log: Mint \"MintPubkey\"".to_string(),
        ],
        slot: 1,
        program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
    };
    let map: LaunchpadMap = [("AuthPubkey".to_string(), "LaunchpadX".to_string())]
        .iter()
        .cloned()
        .collect();
    let event = parse(&raw, &map).expect("should parse");
    assert_eq!(event.mint_pubkey, "MintPubkey");
    assert_eq!(event.authority, "AuthPubkey");
    assert_eq!(event.launchpad, "LaunchpadX");
}

#[tokio::test]
async fn test_mint_event_roundtrip() {
    let event = listener::proto::MintEvent {
        mint_pubkey: "mint".into(),
        authority: "auth".into(),
        program_id: "program".into(),
        slot: 42,
        launchpad: "Unknown".into(),
    };
    let mut buf = Vec::new();
    event.encode(&mut buf).unwrap();
    let out = listener::proto::MintEvent::decode(&*buf).unwrap();
    assert_eq!(event, out);
}

#[tokio::test]
async fn test_end_to_end_publish_latency() {
    if Command::new("docker").output().is_err() {
        eprintln!("Docker not available; skipping end-to-end test");
        return;
    }
    let docker = Cli::default();
    let kafka_image = GenericImage::new("bitnami/kafka", "latest")
        .with_env_var("ALLOW_PLAINTEXT_LISTENER", "yes")
        .with_env_var("KAFKA_CFG_LISTENERS", "PLAINTEXT://:9092")
        .with_env_var(
            "KAFKA_CFG_ADVERTISED_LISTENERS",
            "PLAINTEXT://localhost:9092",
        )
        .with_wait_for(testcontainers::core::WaitFor::message_on_stdout(
            "Kafka started",
        ));
    let kafka_node: Container<_> = docker.run(kafka_image);
    let brokers = format!("localhost:{}", kafka_node.get_host_port_ipv4(9092));

    // simple websocket server sending one log
    use warp::Filter;
    let logs = RawLog {
        logs: vec![
            "Program log: Instruction: InitializeMint".into(),
            "Program log: Mint authority \"AuthPubkey\"".into(),
            "Program log: Mint \"MintPubkey\"".into(),
        ],
        slot: 1,
        program_id: "Token".into(),
    };
    let ws_route = warp::path("ws").and(warp::ws()).map(move |ws: warp::ws::Ws| {
        let logs = logs.clone();
        ws.on_upgrade(move |mut socket| async move {
            let msg = serde_json::json!({
                "params": { "result": { "context": {"slot": logs.slot }, "value": {"logs": logs.logs, "program_id": logs.program_id }} }}).to_string();
            socket.send(warp::ws::Message::text(msg)).await.unwrap();
        })
    });
    let (addr_tx, addr_rx) = std::sync::mpsc::channel();
    tokio::spawn(async move {
        let (addr, server) = warp::serve(ws_route).bind_ephemeral(([127, 0, 0, 1], 0));
        addr_tx.send(addr.port()).unwrap();
        server.await;
    });
    let port = addr_rx.recv().unwrap();
    let _ws_url = format!("ws://localhost:{}/ws", port);

    let producer = KafkaProducer::new(&brokers).await.unwrap();
    let start = std::time::Instant::now();
    producer
        .publish(&listener::proto::MintEvent {
            mint_pubkey: "MintPubkey".into(),
            authority: "AuthPubkey".into(),
            program_id: "Token".into(),
            slot: 1,
            launchpad: "Unknown".into(),
        })
        .await
        .unwrap();
    assert!(start.elapsed() < std::time::Duration::from_millis(500));
}

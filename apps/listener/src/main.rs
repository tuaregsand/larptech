use listener::{kafka::KafkaProducer, parser, rpc};

use futures_util::StreamExt;
use prometheus_exporter;
use std::env;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let helius_ws = env::var("HELIUS_WS").expect("HELIUS_WS must be set");
    let brokers = env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS must be set");
    let map_path = env::var("LAUNCHPAD_MAP").unwrap_or_else(|_| "LAUNCHPAD_MAP.toml".to_string());

    tracing_subscriber::fmt()
        .json()
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .init();

    let _exporter = prometheus_exporter::start("0.0.0.0:9898".parse().unwrap())?;

    let producer = KafkaProducer::new(&brokers).await?;
    let mut stream = Box::pin(rpc::connect_and_stream(&helius_ws).await?);
    let launchpad_map = parser::load_map(&map_path)?;

    while let Some(raw) = stream.next().await {
        if let Some(event) = parser::parse(&raw, &launchpad_map) {
            if let Err(e) = producer.publish(&event).await {
                tracing::error!("kafka error: {}", e);
            }
        }
    }

    Ok(())
}

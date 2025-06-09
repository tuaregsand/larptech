use crate::proto::MintEvent;
use prost::Message;
use rdkafka::{
    client::DefaultClientContext,
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};

pub struct KafkaProducer {
    producer: FutureProducer<DefaultClientContext>,
}

impl KafkaProducer {
    pub async fn new(brokers: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()?;
        Ok(Self { producer })
    }

    pub async fn publish(&self, event: &MintEvent) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        event.encode(&mut buf)?;
        self.producer
            .send(
                FutureRecord::to("token.mints.v1")
                    .payload(&buf)
                    .key(&event.mint_pubkey),
                Timeout::Never,
            )
            .await
            .map_err(|(e, _)| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(())
    }
}

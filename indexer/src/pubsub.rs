use kafka::producer::{Producer, RequiredAcks};
use log::debug;
use std::time::Duration;

pub fn init_producer() -> Producer {
    let host = std::env::var("KAFKA_HOST").unwrap_or("localhost:29092".to_owned());
    debug!("Using Kafka host: {}", host);
    let producer = Producer::from_hosts(vec![host.to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();
    producer
}

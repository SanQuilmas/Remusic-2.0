use crate::env_variables::KAFKA_BROKER;
use log::info;
use rdkafka::{
    ClientConfig,
    consumer::{Consumer, StreamConsumer},
    message::Message,
};
use std::collections::HashMap;
use tokio::time::{Duration, timeout};

pub async fn kafka_consume_messages(topic_name: &str) -> HashMap<i32, String> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set(
            "group.id",
            &format!("rust-actix-consumer-{}", uuid::Uuid::new_v4()),
        )
        .set("bootstrap.servers", KAFKA_BROKER)
        .set("enable.partition.eof", "false")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[topic_name])
        .expect("Can't subscribe to specified topic");

    let mut results = HashMap::new();

    let timeout_secs: u64 = 2;

    let start = tokio::time::Instant::now();

    while start.elapsed().as_secs() < timeout_secs {
        match timeout(Duration::from_millis(500), consumer.recv()).await {
            Ok(Ok(msg)) => {
                if let (Some(key), Some(payload)) = (msg.key(), msg.payload()) {
                    if let (Ok(key_str), Ok(value_str)) =
                        (std::str::from_utf8(key), std::str::from_utf8(payload))
                    {
                        if let Ok(key_int) = key_str.parse::<i32>() {
                            results.insert(key_int, value_str.to_string());
                        }
                    }
                }
                info!("Kafka message read");
            }
            Ok(Err(e)) => {
                info!("Kafka error: {}", e);
                continue;
            }
            Err(_) => {
                // No message received in 500 ms
                continue;
            }
        }
    }
    results
}

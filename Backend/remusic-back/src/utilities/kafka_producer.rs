use std::time::Duration;

use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
};

use crate::env_variables::KAFKA_BROKER;

pub async fn kafka_produce_message(
    topic_name: &str,
    message: &str,
    key: i32,
) -> Result<(), rdkafka::error::KafkaError> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", KAFKA_BROKER)
        .create()
        .expect("Failed to create Kafka producer");

    let key_string = key.to_string();

    let record = FutureRecord::to(topic_name)
        .payload(message)
        .key(&key_string);

    let result = producer.send(record, Duration::from_secs(0)).await;

    match result {
        Ok(_) => {
            println!("Kafka message sent to topic '{}'", topic_name);
            Ok(())
        }
        Err((err, _)) => {
            eprintln!("Failed to send Kafka message: {}", err);
            Err(err)
        }
    }
}

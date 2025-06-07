"""Module with all the kafka functions used in the main script"""

import time

from kafka import KafkaConsumer, KafkaProducer
from kafka.errors import NoBrokersAvailable

from env_variables import (
    DONE_TOPIC,
    ERROR_TOPIC,
    INPROGRESS_TOPIC,
    KAFKA_BROKER,
    POST_URL,
    WAIT_TIME,
)


def update_done_keys_from_topic(done_keys):
    """Function that continuosly updates keys from finished tasks."""
    consumer = KafkaConsumer(
        DONE_TOPIC,
        group_id="done-tracker",
        bootstrap_servers=[KAFKA_BROKER],
        auto_offset_reset="earliest",
    )
    for message in consumer:
        key_str = message.key.decode("utf-8")
        if key_str not in done_keys:
            done_keys.add(key_str)
            print(f"Adding done key: {key_str}")


def update_in_progress_keys_from_topic(done_keys):
    """Function that continuosly updates keys from in progress tasks."""
    consumer = KafkaConsumer(
        INPROGRESS_TOPIC,
        group_id="in-progress-tracker",
        bootstrap_servers=[KAFKA_BROKER],
        auto_offset_reset="earliest",
    )
    for message in consumer:
        key_str = message.key.decode("utf-8")
        if key_str not in done_keys:
            done_keys.add(key_str)
            print(f"Adding in progress key: {key_str}")


def get_processed_keys():
    """Function that updates keys from in progress and done tasks once at startup."""
    in_progress_consumer = KafkaConsumer(
        INPROGRESS_TOPIC,
        bootstrap_servers=[KAFKA_BROKER],
        auto_offset_reset="earliest",
        consumer_timeout_ms=1000,
    )

    done_consumer = KafkaConsumer(
        DONE_TOPIC,
        bootstrap_servers=[KAFKA_BROKER],
        auto_offset_reset="earliest",
        consumer_timeout_ms=1000,
    )

    done_keys = set()

    for message in in_progress_consumer:
        done_keys.add(message.key.decode("utf-8"))

    for message in done_consumer:
        done_keys.add(message.key.decode("utf-8"))

    return done_keys


def wait_for_kafka_consumer(*args, **kwargs):
    """Function that waits for a broker to be available before creating a kafka consumer."""
    while True:
        try:
            return KafkaConsumer(*args, **kwargs)
        except NoBrokersAvailable:
            print(f"Kafka broker not available yet, retrying in {WAIT_TIME} seconds...")
            time.sleep(WAIT_TIME)


def wait_for_kafka_producer(*args, **kwargs):
    """Function that waits for a broker to be available before creating a kafka producer."""
    while True:
        try:
            return KafkaProducer(*args, **kwargs)
        except NoBrokersAvailable:
            print(f"Kafka broker not available yet, retrying in {WAIT_TIME} seconds...")
            time.sleep(WAIT_TIME)


def send_error_status_to_kafka(key_str, message, producer):
    """Function that sends error info to correct kafka topic."""
    producer.send(ERROR_TOPIC, key=message.key, value=message.value)
    producer.flush()
    return f"ERROR STATUS of {key_str} published to {ERROR_TOPIC}"


def process_kafka_message(message, producer, done_keys):
    """Function that checks if current task is already done or being worked on."""
    key_str = message.key.decode("utf-8")
    print(f"Checking key: {key_str} against done_keys: {done_keys}")
    if key_str in done_keys:
        print(f"Skipping {POST_URL + key_str}")
        return False

    return send_in_progress_status_to_kafka(
        message.key, message.value, key_str, producer
    )


def send_in_progress_status_to_kafka(key_val, message_val, key_str, producer):
    """Sends an update to in progress topic"""
    producer.send(INPROGRESS_TOPIC, key=key_val, value=message_val)
    producer.flush()
    print(f"In progress STATUS of {key_str} published to {INPROGRESS_TOPIC}")
    return True

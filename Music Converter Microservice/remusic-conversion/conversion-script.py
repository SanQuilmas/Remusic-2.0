import json
from confluent_kafka import Consumer, Producer
import requests
import subprocess
from env_variables import (
    KAFKA_BROKER,
    INPUT_TOPIC,
    INPROGRESS_TOPIC,
    GROUP_ID,
    POST_URL,
    OEmer_BINARY
)

# 1. Build a set of done/in-progress keys
def get_processed_keys():
    consumer = Consumer({
        'bootstrap.servers': KAFKA_BROKER,
        'group.id': GROUP_ID + '_dedupe',
        'auto.offset.reset': 'earliest'
    })
    consumer.subscribe([INPROGRESS_TOPIC])
    done_keys = set()
    while True:
        msg = consumer.poll(1.0)
        if msg is None:
            break
        if msg.error():
            continue
        data = json.loads(msg.value().decode('utf-8'))
        done_keys.add(data['key'])
    consumer.close()
    return done_keys

# 2. Main worker loop
def main():
    done_keys = get_processed_keys()
    consumer = Consumer({
        'bootstrap.servers': KAFKA_BROKER,
        'group.id': GROUP_ID,
        'auto.offset.reset': 'earliest'
    })
    producer = Producer({'bootstrap.servers': KAFKA_BROKER})
    consumer.subscribe([INPUT_TOPIC])

    while True:
        msg = consumer.poll(1.0)
        if msg is None:
            continue
        if msg.error():
            print("Consumer error: {}".format(msg.error()))
            continue
        job = json.loads(msg.value().decode('utf-8'))
        key = job['key']
        data = job['data']
        if key in done_keys:
            continue
        # Publish to in-progress topic
        producer.produce(INPROGRESS_TOPIC, json.dumps({'key': key, 'data': data}).encode('utf-8'))
        producer.flush()
        # Run oemer
        subprocess.run([OEmer_BINARY, "--input", data], check=True)
        # POST request
        requests.post(POST_URL, json={'key': key, 'data': data})
        done_keys.add(key)

if __name__ == '__main__':
    main()
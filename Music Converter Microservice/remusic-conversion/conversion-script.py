import json
from kafka import KafkaConsumer
import requests
import base64
from env_variables import (
    KAFKA_BROKER,
    INPUT_TOPIC,
    INPROGRESS_TOPIC,
    GROUP_ID,
    POST_URL,
    DONE_TOPIC,
    OEMER_BINARY
)

def get_processed_keys():
    in_progress_consumer = KafkaConsumer(INPROGRESS_TOPIC,
                         group_id='my-group',
                         bootstrap_servers=[KAFKA_BROKER],
                         auto_offset_reset='earliest',
                         consumer_timeout_ms=1000)

    done_consumer = KafkaConsumer(DONE_TOPIC,
                         group_id='my-group',
                         bootstrap_servers=[KAFKA_BROKER],
                         auto_offset_reset='earliest',
                         consumer_timeout_ms=1000)
    done_keys = set()

    for message in in_progress_consumer:
        done_keys.add(message.key.decode('utf-8'))
        pass

    for message in done_consumer:
        done_keys.add(message.key.decode('utf-8'))
        pass

    return done_keys

def encode_file_to_base64(file_path):
    with open(file_path, "rb") as f:
        return base64.b64encode(f.read()).decode("utf-8")

def main():
    consumer = KafkaConsumer(INPUT_TOPIC,
                         group_id='my-group',
                         bootstrap_servers=[KAFKA_BROKER],
                         auto_offset_reset='earliest')

    for message in consumer:
        done_keys = get_processed_keys()
        key_str = message.key.decode('utf-8') if message.key else ""

        if key_str in done_keys:
            print(f"Skipping {POST_URL + key_str}")
            continue

        print(f"{message.topic} key:{key_str} value: {message.value.decode('utf-8')}")
        print(f"Working on {POST_URL + key_str}")

        music_xml_path = './test_files/MozaVeilSample.xml'
        midi_path = './test_files/melody.mid'

        music_xml_base64 = encode_file_to_base64(music_xml_path)
        midi_base64 = encode_file_to_base64(midi_path)

        payload = {
            'id': int(key_str),
            'music_xml_blob': music_xml_base64,
            'midi_blob': midi_base64
        }

        # Publish to in-progress topic
        # producer.produce(INPROGRESS_TOPIC, json.dumps({'key': key, 'data': payload}).encode('utf-8'))
        # # Run oemer
        # subprocess.run([OEMER_BINARY, "--input", data], check=True)
        # # POST request
        # requests.post(POST_URL, json={'key': key, 'data': data})
        # done_keys.add(key)

        response = requests.put(POST_URL + key_str, json=payload)
        print(f"PUT {POST_URL + key_str} status: {response.status_code}, response: {response.text}")

if __name__ == '__main__':
    main()

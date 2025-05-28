import json
import os
from kafka import KafkaConsumer, KafkaProducer
import requests
import base64
import subprocess
from env_variables import (
    KAFKA_BROKER,
    INPUT_TOPIC,
    INPROGRESS_TOPIC,
    GROUP_ID,
    POST_URL,
    DONE_TOPIC,
    OEMER_BINARY
)

def download_image(image_url, save_dir):
    os.makedirs(save_dir, exist_ok=True)
    filename = os.path.basename(image_url)
    save_path = os.path.join(save_dir, filename)

    response = requests.get(image_url, stream=True)
    response.raise_for_status()  # Raise error if download fails

    with open(save_path, 'wb') as f:
        for chunk in response.iter_content(chunk_size=8192):
            f.write(chunk)

    return save_path

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

done_keys = get_processed_keys()
print(f"Loaded processed keys: {done_keys}")

def main():
    consumer = KafkaConsumer(INPUT_TOPIC,
                         group_id='my-group',
                         bootstrap_servers=[KAFKA_BROKER],
                         auto_offset_reset='earliest')


    producer = KafkaProducer(bootstrap_servers=[KAFKA_BROKER])

    for message in consumer:
        key_str = message.key.decode('utf-8')


        print(f"Checking key: {key_str} against done_keys: {done_keys}")
        if key_str in done_keys:
            print(f"Skipping {POST_URL + key_str}")
            continue

        producer.send(INPROGRESS_TOPIC, key=message.key, value=message.value)
        producer.flush()
        done_keys.add(key_str)
        print(f"In progress STATUS of {key_str} published to {INPROGRESS_TOPIC}")

        value_str = message.value.decode('utf-8')
        data = json.loads(value_str)
        image_url = data.get("image_path")

        print(f"{message.topic} key:{key_str} value: {value_str}")
        print(f"Working on {POST_URL + key_str}")

        image_path = download_image(image_url, "/app/temp/" + key_str)
        print(f"Downloaded image to {image_path}")

        music_xml_path = f"/app/temp/{key_str}/music_xml/{key_str}_musicxml.musicxml"
        midi_path = f"/app/temp/{key_str}/midi/{key_str}_midi.mid"

        print(f"Starting conversion process image->musicxml for {image_path}")
        command = ["oemer", "-o", music_xml_path, image_path]
        print(command)
        result = subprocess.run(command, capture_output=True, check=True)

        print(f"Starting conversion process musicxml->midi for {music_xml_path}")
        command = ["python", "convert_xml_to_midi.py", f"/app/temp/{key_str}/music_xml/", f"/app/temp/{key_str}/midi/"]
        print(command)
        result = subprocess.run(command, capture_output=True, check=True)

        music_xml_base64 = encode_file_to_base64(music_xml_path)
        midi_base64 = encode_file_to_base64(midi_path)

        payload = {
            'id': int(key_str),
            'music_xml_blob': music_xml_base64,
            'midi_blob': midi_base64
        }
        print(f"Sending finished conversions to backend")
        response = requests.put(POST_URL + key_str, json=payload)
        print(f"PUT {POST_URL + key_str} status: {response.status_code}, response: {response.text}")

if __name__ == '__main__':
    main()

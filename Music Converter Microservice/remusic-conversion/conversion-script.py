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
    OEMER_BINARY,
    ERROR_TOPIC
)
import uuid
import threading

done_keys = set()

def update_done_keys_from_topic():
    global done_keys
    consumer = KafkaConsumer(DONE_TOPIC,
                             group_id="done-tracker",
                             bootstrap_servers=[KAFKA_BROKER],
                             auto_offset_reset='earliest')
    for message in consumer:
        key_str = message.key.decode('utf-8')
        done_keys.add(key_str)
        print(f"Adding done key: {key_str}")

def update_in_progress_keys_from_topic():
    global done_keys
    consumer = KafkaConsumer(INPROGRESS_TOPIC,
                             group_id="in-progress-tracker",
                             bootstrap_servers=[KAFKA_BROKER],
                             auto_offset_reset='earliest')
    for message in consumer:
        key_str = message.key.decode('utf-8')
        done_keys.add(key_str)
        print(f"Adding in progress key: {key_str}")

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
                         bootstrap_servers=[KAFKA_BROKER],
                         auto_offset_reset='earliest',
                         consumer_timeout_ms=1000)

    done_consumer = KafkaConsumer(DONE_TOPIC,
                         bootstrap_servers=[KAFKA_BROKER],
                         auto_offset_reset='earliest',
                         consumer_timeout_ms=1000)
    done_keys = set()

    for message in in_progress_consumer:
        done_keys.add(message.key.decode('utf-8'))

    for message in done_consumer:
        done_keys.add(message.key.decode('utf-8'))

    return done_keys

def encode_file_to_base64(file_path):
    with open(file_path, "rb") as f:
        return base64.b64encode(f.read()).decode("utf-8")

def main():
    consumer = KafkaConsumer(INPUT_TOPIC,
                         group_id=f"my-group",
                         bootstrap_servers=[KAFKA_BROKER],
                         auto_offset_reset='earliest')


    producer = KafkaProducer(bootstrap_servers=[KAFKA_BROKER])

    threading.Thread(target=update_done_keys_from_topic, daemon=True).start()
    threading.Thread(target=update_in_progress_keys_from_topic, daemon=True).start()

    print("Starting up conversion middleware ...")

    global done_keys
    done_keys.update(get_processed_keys())
    print(f"Loaded processed keys: {done_keys}")

    for message in consumer:
        key_str = message.key.decode('utf-8')

        # done_keys.update(get_processed_keys())

        print(f"Checking key: {key_str} against done_keys: {done_keys}")
        if key_str in done_keys:
            print(f"Skipping {POST_URL + key_str}")
            continue

        producer.send(INPROGRESS_TOPIC, key=message.key, value=message.value)
        producer.flush()

        # done_keys.add(key_str)
        print(f"In progress STATUS of {key_str} published to {INPROGRESS_TOPIC}")

        value_str = message.value.decode('utf-8')
        data = json.loads(value_str)
        image_url = data.get("image_path")

        print(f"{message.topic} key:{key_str} value: {value_str}")
        print(f"Working on {POST_URL + key_str}")

        image_path = download_image(image_url, "/app/temp/" + key_str)
        print(f"Downloaded image to {image_path}")

        os.makedirs(f"/app/temp/{key_str}/music_xml/", exist_ok=True)
        os.makedirs(f"/app/temp/{key_str}/midi/", exist_ok=True)

        music_xml_path = f"/app/temp/{key_str}/music_xml/{key_str}_musicxml.musicxml"
        midi_path = f"/app/temp/{key_str}/midi/{key_str}_musicxml.mid"

        GLOBAL_ERROR = False

        try:
            print(f"Starting conversion process image->musicxml for {image_path}")
            command_music_xml = ["oemer", "-o", music_xml_path, image_path]
            print(command_music_xml)
            result = subprocess.run(command_music_xml, capture_output=True, check=True)
        except subprocess.CalledProcessError as e:
            print(f"Error running command: {command_music_xml}")
            print("Return code:", e.returncode)
            GLOBAL_ERROR = True
        except Exception as e:
            print(f"Unexpected error: {e}")

        try:
            print(f"Starting conversion process musicxml->midi for {music_xml_path}")
            command_midi = ["python", "convert_xml_to_midi.py", f"/app/temp/{key_str}/music_xml/", f"/app/temp/{key_str}/midi/"]
            print(command_midi)
            result = subprocess.run(command_midi, capture_output=True, check=True)
        except subprocess.CalledProcessError as e:
            print(f"Error running command: {command_midi}")
            print("Return code:", e.returncode)
            GLOBAL_ERROR = True
        except Exception as e:
            print(f"Unexpected error: {e}")

        if not GLOBAL_ERROR:
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
        else:
            producer.send(ERROR_TOPIC, key=message.key, value=message.value)
            producer.flush()
            print(f"ERROR STATUS of {key_str} published to {ERROR_TOPIC}")

if __name__ == '__main__':
    main()

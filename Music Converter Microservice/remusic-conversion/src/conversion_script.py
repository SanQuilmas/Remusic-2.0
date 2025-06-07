"""Script providing conversion functionality for ReMusic."""

import json
import os
import threading

from env_variables import (
    GROUP_ID,
    INPUT_TOPIC,
    KAFKA_BROKER,
    POST_URL,
)
from kafka_functions import (
    get_processed_keys,
    process_kafka_message,
    send_error_status_to_kafka,
    update_done_keys_from_topic,
    update_in_progress_keys_from_topic,
    wait_for_kafka_consumer,
    wait_for_kafka_producer,
)
from utility_functions import (
    download_image,
    run_subprocess,
    send_payload_to_server,
)


def prepare_image_and_paths(key_str, image_url):
    """Function that creates the file folders and assings paths."""
    image_path = download_image(image_url, f"/app/temp/{key_str}")
    print(f"Downloaded image to {image_path}")

    music_xml_dir = f"/app/temp/{key_str}/music_xml/"
    midi_dir = f"/app/temp/{key_str}/midi/"
    os.makedirs(music_xml_dir, exist_ok=True)
    os.makedirs(midi_dir, exist_ok=True)

    music_xml_path = f"{music_xml_dir}{key_str}_musicxml.musicxml"
    midi_path = f"{midi_dir}{key_str}_musicxml.mid"
    return image_path, music_xml_path, midi_path


def main():
    """Main Function."""

    consumer = wait_for_kafka_consumer(
        INPUT_TOPIC,
        group_id=GROUP_ID,
        bootstrap_servers=[KAFKA_BROKER],
        auto_offset_reset="earliest",
    )

    producer = wait_for_kafka_producer(bootstrap_servers=[KAFKA_BROKER])

    done_keys = set()

    threading.Thread(
        target=update_done_keys_from_topic, args=(done_keys,), daemon=True
    ).start()
    threading.Thread(
        target=update_in_progress_keys_from_topic, args=(done_keys,), daemon=True
    ).start()

    print("Starting up conversion middleware ...")

    done_keys.update(get_processed_keys())
    print(f"Loaded processed keys: {done_keys}")

    for message in consumer:
        if not process_kafka_message(message, producer, done_keys):
            continue

        key_str = message.key.decode("utf-8")
        value_str = message.value.decode("utf-8")
        data = json.loads(value_str)
        image_url = data.get("image_path")

        print(f"{message.topic} key:{key_str} value: {value_str}")
        print(f"Working on {POST_URL + key_str}")

        image_path, music_xml_path, midi_path = prepare_image_and_paths(
            key_str, image_url
        )

        global_error = False

        command_music_xml = ["oemer", "-o", music_xml_path, "--use-tf", image_path]
        global_error |= run_subprocess(
            command=command_music_xml,
            description=f"conversion process image->musicxml for {image_path}",
        )

        command_midi = [
            "python",
            "convert_xml_to_midi.py",
            f"/app/temp/{key_str}/music_xml/",
            f"/app/temp/{key_str}/midi/",
        ]
        global_error |= run_subprocess(
            command=command_midi,
            description=f"conversion process musicxml->midi for {music_xml_path}",
        )

        if not global_error:
            print(send_payload_to_server(key_str, music_xml_path, midi_path))
        else:
            print(send_error_status_to_kafka(key_str, message, producer))


if __name__ == "__main__":
    main()

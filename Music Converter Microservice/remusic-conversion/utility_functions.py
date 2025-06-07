"""Module with varios utility functions for file and process handling"""

import base64
import os
import subprocess

import requests

from env_variables import POST_URL


def download_image(image_url, save_dir):
    """Function that downloads the image to be converted into local storage."""
    os.makedirs(save_dir, exist_ok=True)
    filename = os.path.basename(image_url)
    save_path = os.path.join(save_dir, filename)

    response = requests.get(image_url, stream=True, timeout=10)
    response.raise_for_status()

    with open(save_path, "wb") as f:
        for chunk in response.iter_content(chunk_size=8192):
            f.write(chunk)

    return save_path


def encode_file_to_base64(file_path):
    """Function that encodes files to base64 to send them to the api."""
    with open(file_path, "rb") as f:
        return base64.b64encode(f.read()).decode("utf-8")


def run_subprocess(command, description):
    """Function that runs the necesary scripts."""
    try:
        print(f"Starting {description} | {' '.join(command)}")
        subprocess.run(command, capture_output=True, check=True)
        return False
    except subprocess.CalledProcessError as e:
        print(f"Error running command: {command}")
        print("Return code:", e.returncode)
    except OSError as e:
        print(f"OS error: {e}")
    except Exception as e:
        print(f"Unexpected error: {e}")
        raise
    return True


def send_payload_to_server(key_str, music_xml_path, midi_path):
    """Function that sends finished midi and musicxml files to api."""
    music_xml_base64 = encode_file_to_base64(music_xml_path)
    midi_base64 = encode_file_to_base64(midi_path)
    payload = {
        "id": int(key_str),
        "music_xml_blob": music_xml_base64,
        "midi_blob": midi_base64,
    }
    print("Sending finished conversions to backend")
    response = requests.put(POST_URL + key_str, json=payload, timeout=10)
    return f"PUT-{POST_URL + key_str} status-{response.status_code} response-{response.text}"

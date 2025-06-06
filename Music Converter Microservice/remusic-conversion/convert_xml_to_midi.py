"""Convert musicxml to midi."""

import argparse
import os

from music21 import converter, midi


def process_mxl_file(input_file, input_dir, output_dir, verbose):
    """Function that converts MusicXml file to Midi."""
    try:
        if verbose:
            print(f"processing: {input_file}")
        score = converter.parse(input_file)

        relative_path = os.path.relpath(os.path.dirname(input_file), input_dir)
        specific_output_dir = os.path.join(output_dir, relative_path)

        os.makedirs(specific_output_dir, exist_ok=True)

        file_name = os.path.splitext(os.path.basename(input_file))[0]
        midi_path = os.path.join(specific_output_dir, f"{file_name}.mid")

        combined_midi = midi.MidiFile()

        for part in score.parts:
            mf = midi.translate.streamToMidiFile(part)
            for track in mf.tracks:
                combined_midi.tracks.append(track)

        combined_midi.open(midi_path, "wb")
        combined_midi.write()
        combined_midi.close()

    except Exception as e:
        print(f"Error processing {input_file}: {e}")
        raise


def main():
    """Function that handles file work."""
    parser = argparse.ArgumentParser(
        description="Process MusicXML files and convert to MIDI."
    )
    parser.add_argument(
        "input_dir", type=str, help="Directory containing MusicXML files"
    )
    parser.add_argument("output_dir", type=str, help="Directory to save MIDI files")
    parser.add_argument("--verbose", action="store_true", help="enable prints")

    args = parser.parse_args()

    input_dir_abs = os.path.abspath(args.input_dir)
    output_dir_abs = os.path.abspath(args.output_dir)

    if output_dir_abs.startswith(input_dir_abs):
        print("Output directory cannot be inside the input directory.")
        return

    if args.verbose:
        print(f"Input Directory: {input_dir_abs}")
        print(f"Output Directory: {output_dir_abs}")

    supported_extensions = (".musicxml", ".xml", ".mxl")
    for root, _, files in os.walk(input_dir_abs):
        if output_dir_abs.startswith(root):
            continue

        for file in files:
            if file.lower().endswith(supported_extensions):
                input_file = os.path.join(root, file)
                process_mxl_file(
                    input_file, input_dir_abs, output_dir_abs, args.verbose
                )


if __name__ == "__main__":
    main()

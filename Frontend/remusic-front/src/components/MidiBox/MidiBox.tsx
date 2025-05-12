import { useContext } from "react";
import { MidiPlayerVisualizer } from "../MidiPlayerVisualizer/MidiPlayerVisualizer";
import { instanceContext } from "../MainMusicContainer/MainMusicContainer";

export const MidiBox = () => {
  const { instance } = useContext(instanceContext);
  return (
    <div>
      {instance && instance.midi_path ? (
        <MidiPlayerVisualizer music_midi={instance.midi_path} />
      ) : (
        ""
      )}
    </div>
  );
};

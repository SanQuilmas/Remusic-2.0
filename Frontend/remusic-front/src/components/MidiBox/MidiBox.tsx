import { useContext } from "react";
import { InstanceContext } from "../../data/contexts/InstanceContext";
import { MidiPlayerVisualizer } from "../MidiPlayerVisualizer/MidiPlayerVisualizer";

export const MidiBox = () => {
  const { instance } = useContext(InstanceContext);
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

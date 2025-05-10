import { useContext } from "react";
import { MidiPlayerVisualizer } from "../MidiPlayerVisualizer/MidiPlayerVisualizer";
import { instanceContext } from "../MainMusicContainer/MainMusicContainer";

export const MidiBox = () => {
  const { music_midi } = useContext(instanceContext);
  return (
    <div>
      <MidiPlayerVisualizer music_midi={music_midi} />
    </div>
  );
};

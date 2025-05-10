import { PlayerElement, VisualizerElement } from "html-midi-player";
import { useEffect, useRef } from "react";

interface MidiPlayerVisualizerProp {
  music_midi: string;
}

export const MidiPlayerVisualizer = ({
  music_midi,
}: MidiPlayerVisualizerProp) => {
  const playerRef = useRef<HTMLDivElement>(null);
  const visualizerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Create instances of the Web Components
    const player = new PlayerElement();
    const visualizer = new VisualizerElement();

    // Set properties
    player.src = music_midi;
    player.soundFont = "";
    visualizer.type = "piano-roll";

    if (playerRef.current) {
      playerRef.current.appendChild(player);
    }
    if (visualizerRef.current) {
      visualizerRef.current.appendChild(visualizer);
    }

    // Connect visualizer to player
    player.addVisualizer(visualizer);

    // Cleanup on unmount
    return () => {
      if (playerRef.current) playerRef.current.removeChild(player);
      if (visualizerRef.current) visualizerRef.current.removeChild(visualizer);
    };
  }, [music_midi]);

  return (
    <div>
      <div ref={playerRef}></div>
      <div ref={visualizerRef}></div>
    </div>
  );
};

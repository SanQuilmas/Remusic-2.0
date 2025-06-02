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

    const playerCurrent = playerRef.current;
    const visualizerCurrent = visualizerRef.current;

    if (playerCurrent) {
      playerCurrent.appendChild(player);
    }
    if (visualizerCurrent) {
      visualizerCurrent.appendChild(visualizer);
    }

    // Connect visualizer to player
    player.addVisualizer(visualizer);

    // Cleanup on unmount
    return () => {
      if (playerCurrent) playerCurrent.removeChild(player);
      if (visualizerCurrent) visualizerCurrent.removeChild(visualizer);
    };
  }, [music_midi]);

  return (
    <div>
      <div ref={playerRef}></div>
      <div ref={visualizerRef}></div>
    </div>
  );
};

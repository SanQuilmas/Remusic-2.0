import { createContext } from "react";
import { MidiBox } from "../MidiBox/MidiBox";
import { SheetMusicBox } from "../SheetMusicBox/SheetMusicBox";
import "./MainMusicContainer.css";

interface instanceContextType {
  music_midi: string;
  music_xml: string;
}

const instanceContext = createContext<instanceContextType>({
  music_xml: "",
  music_midi: "",
});

export const MainMusicContainer = () => {
  return (
    <instanceContext.Provider
      value={{
        music_xml: "/MozaVeilSample.xml",
        music_midi:
          "https://magenta.github.io/magenta-js/music/demos/melody.mid",
      }}
    >
      <h1> Title </h1>
      <div className="music_details">
        <div className="musicxml_column music_details_column">
          <div>
            <a href="#"> Download MusicXml </a>
          </div>

          <div>
            <SheetMusicBox />
          </div>
        </div>

        <div className="musicmidi_column music_details_column">
          <div>
            <a href="#"> Download MIDI </a>
          </div>

          <div>
            <MidiBox />
          </div>
        </div>
      </div>
    </instanceContext.Provider>
  );
};

export { instanceContext };

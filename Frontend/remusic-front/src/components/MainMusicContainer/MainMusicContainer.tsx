import { createContext, useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { MainAPI } from "../../data/api_endpoints/enpoints";
import type { instanciaPartitura } from "../../data/entities_types/types";
import { MidiBox } from "../MidiBox/MidiBox";
import { SheetMusicBox } from "../SheetMusicBox/SheetMusicBox";
import "./MainMusicContainer.css";
import { sanitizePath } from "../../utilities/StaticURLSanitize";

interface instanceContextType {
  instance: instanciaPartitura | null;
}

const instanceContext = createContext<instanceContextType>({
  instance: null,
});

export const MainMusicContainer = () => {
  const [instance, setInstance] = useState<instanciaPartitura | null>(null);
  const instanceID = useParams();

  const fetchData = async () => {
    const response = await fetch(MainAPI + `/${instanceID.id}`);
    const data: instanciaPartitura = await response.json();
    const sanitizedData: instanciaPartitura = {
      ...data,
      image_path: sanitizePath(data.image_path),
      music_xml_path: sanitizePath(data.music_xml_path),
      midi_path: sanitizePath(data.midi_path),
    };
    setInstance(sanitizedData);
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <instanceContext.Provider value={{ instance }}>
      <h1> {instance ? instance.name : ""} </h1>
      <div className="music_details">
        <div className="musicxml_column music_details_column">
          <div>
            <a
              href={
                instance && instance.music_xml_path
                  ? instance.music_xml_path
                  : "#"
              }
            >
              {" "}
              Download MusicXml{" "}
            </a>
          </div>

          <div>
            <SheetMusicBox />
          </div>
        </div>

        <div className="musicmidi_column music_details_column">
          <div>
            <a href={instance && instance.midi_path ? instance.midi_path : "#"}>
              {" "}
              Download MIDI{" "}
            </a>
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

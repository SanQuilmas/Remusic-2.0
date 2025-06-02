import { useCallback, useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { MainAPI } from "../../data/api_endpoints/enpoints";
import { InstanceContext } from "../../data/contexts/InstanceContext";
import type { instanciaPartitura } from "../../data/entities_types/types";
import { sanitizePath } from "../../utilities/StaticURLSanitize";
import { MidiBox } from "../MidiBox/MidiBox";
import { SheetMusicBox } from "../SheetMusicBox/SheetMusicBox";
import "./MainMusicContainer.css";

export const MainMusicContainer = () => {
  const [instance, setInstance] = useState<instanciaPartitura | null>(null);
  const instanceID = useParams();

  const fetchData = useCallback(async () => {
    const response = await fetch(MainAPI + `/${instanceID.id}`);
    const data: instanciaPartitura = await response.json();
    const sanitizedData: instanciaPartitura = {
      ...data,
      image_path: sanitizePath(data.image_path),
      music_xml_path: sanitizePath(data.music_xml_path),
      midi_path: sanitizePath(data.midi_path),
    };
    setInstance(sanitizedData);
  }, [instanceID]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  return (
    <InstanceContext.Provider value={{ instance }}>
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
            {instance && instance.music_xml_path ? <SheetMusicBox /> : ""}
          </div>
        </div>

        <div className="musicmidi_column music_details_column">
          <div>
            <a href={instance && instance.midi_path ? instance.midi_path : "#"}>
              {" "}
              Download MIDI{" "}
            </a>
          </div>

          <div>{instance && instance.midi_path ? <MidiBox /> : ""}</div>
        </div>
      </div>
    </InstanceContext.Provider>
  );
};

import { useContext } from "react";
import { InstanceContext } from "../../data/contexts/InstanceContext";
import OpenSheetMusicDisplayComponent from "../OpenSheetMusicDisplayComponent/OpenSheetMusicDisplayComponent";
import "./SheetMusicBox.css";

export const SheetMusicBox = () => {
  const { instance } = useContext(InstanceContext);

  return (
    <div className="music_container">
      {instance && instance.music_xml_path ? (
        <OpenSheetMusicDisplayComponent fileUrl={instance.music_xml_path} />
      ) : (
        ""
      )}
    </div>
  );
};

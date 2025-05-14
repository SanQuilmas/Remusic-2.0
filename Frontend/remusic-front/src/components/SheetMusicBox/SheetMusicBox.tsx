import { useContext } from "react";
import OpenSheetMusicDisplayComponent from "../OpenSheetMusicDisplayComponent/OpenSheetMusicDisplayComponent";
import "./SheetMusicBox.css";
import { instanceContext } from "../MainMusicContainer/MainMusicContainer";

export const SheetMusicBox = () => {
  const { instance } = useContext(instanceContext);

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

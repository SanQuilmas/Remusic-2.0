import { useEffect, useRef } from "react";
import { OpenSheetMusicDisplay } from "opensheetmusicdisplay";

interface opmdProp {
  fileUrl: string;
}

const OpenSheetMusicDisplayComponent = ({ fileUrl }: opmdProp) => {
  const containerRef = useRef(null);

  useEffect(() => {
    if (!containerRef.current) return;

    const osmd = new OpenSheetMusicDisplay(containerRef.current);
    osmd.setOptions({
      backend: "svg",
      drawTitle: true,
    });

    osmd.load(fileUrl).then(() => {
      osmd.render();
    });
  }, [fileUrl]);

  return (
    <div>
      {fileUrl ? <div ref={containerRef} /> : <p>No file URL provided.</p>}
    </div>
  );
};

export default OpenSheetMusicDisplayComponent;

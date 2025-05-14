import { useState } from "react";
import "./UploadBox.css";

export const UploadBox = () => {
  const [valName, setValName] = useState("");
  const [valImg, setValImg] = useState<File | null>(null);

  const handleSubmit = () => {};

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      setValImg(e.target.files[0]);
    }
  };

  return (
    <form className="upload_container" onSubmit={handleSubmit}>
      <input
        type="text"
        name="music_name"
        id="music_name"
        value={valName}
        onChange={(e) => setValName(e.target.value)}
        required
      />
      <input
        type="file"
        accept="image/*"
        capture="environment"
        name="music_img"
        id="music_img"
        onChange={handleFileChange}
        required
      />
      <button type="submit"> Upload and Digitize </button>
    </form>
  );
};

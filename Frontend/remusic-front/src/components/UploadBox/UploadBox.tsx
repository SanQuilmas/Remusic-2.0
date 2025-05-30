import { useState } from "react";
import "./UploadBox.css";
import { MainAPI } from "../../data/api_endpoints/enpoints";
import { useNavigate } from "react-router-dom";

export const UploadBox = () => {
  const [valName, setValName] = useState("");
  const [valImg, setValImg] = useState<File | null>(null);
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    let imgBase64: string | null = null;
    if (valImg) {
      imgBase64 = await new Promise<string | null>((resolve) => {
        const reader = new FileReader();
        reader.onload = () => {
          const result = reader.result as string;
          const base64 = result.split(",")[1];
          resolve(base64);
        };
        reader.onerror = () => resolve(null);
        reader.readAsDataURL(valImg);
      });
    }

    const payload = {
      id: 0,
      name: valName.replace(/\s+/g, "_"),
      image_blob: imgBase64,
    };

    try {
      const response = await fetch(MainAPI, {
        method: "POST",
        headers: {
          "Content-Type": "text/plain",
        },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        // handle error
        console.error("Upload failed - Error: ");
        console.log(response);
        return;
      }
      navigate("/gallery");
    } catch (err) {
      console.error(err);
    }
  };

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

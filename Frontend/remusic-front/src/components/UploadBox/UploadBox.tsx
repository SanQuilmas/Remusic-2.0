import "./UploadBox.css";

export const UploadBox = () => {
  return (
    <div className="upload_container">
      <input type="text" name="music_name" id="music_name" required />
      <input
        type="file"
        accept="image/*"
        capture="environment"
        name="music_img"
        id="music_img"
        required
      />
      <button> Upload and Digitize </button>
    </div>
  );
};

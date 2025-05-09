import "./UploadBox.css"

export const UploadBox = () => {
    return(
        <div className="upload_container">
            <input type="text" name="music_name" id="music_name" />
            <input type="file" name="music_img" id="music_img" />
            <button> Upload and Digitize </button>
        </div>
    )
}
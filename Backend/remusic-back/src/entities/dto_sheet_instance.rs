// use base64::Engine;
use serde::{Deserialize, Serialize};

// use super::sheet_instance::Model;

#[derive(Serialize, Deserialize)]
pub struct DtoCreateSheetInstance {
    pub id: i32,
    pub name: String,
    pub image_blob: String,
}

#[derive(Serialize, Deserialize)]
pub struct DtoUpdateSheetInstance {
    pub id: i32,
    pub music_xml_blob: Option<String>,
    pub midi_blob: Option<String>,
}

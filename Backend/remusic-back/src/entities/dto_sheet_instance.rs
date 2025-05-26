// use base64::Engine;
use serde::{Deserialize, Serialize};

// use super::sheet_instance::Model;

#[derive(Serialize, Deserialize)]
pub struct DtoCreateSheetInstance {
    pub id: i32,
    pub name: String,
    pub image_blob: String,
}

// impl From<Model> for DtoCreateSheetInstance {
//     fn from(model: Model) -> Self {
//         DtoCreateSheetInstance {
//             id: model.id,
//             name: model.name,
//             image_blob: model.image_path,
//         }
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct DtoUpdateSheetInstance {
    pub id: i32,
    pub music_xml_blob: Option<Vec<u8>>,
    pub midi_blob: Option<Vec<u8>>,
}

// impl From<Model> for DtoUpdateSheetInstance {
//     fn from(model: Model) -> Self {
//         DtoUpdateSheetInstance {
//             id: model.id,
//             music_xml_blob: model.music_xml_blob,
//             midi_blob: model.midi_blob,
//         }
//     }
// }

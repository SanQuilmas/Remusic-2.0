use base64::Engine;
use serde::{Deserialize, Serialize};

use super::sheet_instance::Model;

#[derive(Serialize, Deserialize)]
pub struct DtoCreateSheetInstance {
    pub id: i32,
    pub name: String,
    pub image_blob: String,
}

impl From<Model> for DtoCreateSheetInstance {
    fn from(model: Model) -> Self {
        DtoCreateSheetInstance {
            id: model.id,
            name: model.name,
            image_blob: base64::engine::general_purpose::STANDARD.encode(&model.image_blob),
        }
    }
}

use crate::entities;
use crate::entities::sheet_instance::Model;
use crate::entities::{prelude::*, *};
use sea_orm::*;

// let mut temp =
//         SheetInstance::build_sheet_instance(id, "Yo".to_owned(), "image_path".to_owned());
//     temp._set_midi_path(String::from(
//         "https://magenta.github.io/magenta-js/music/demos/melody.mid",
//     ));
//     temp._set_musicxml_path(String::from("/MozaVeilSample.xml"));

pub fn find_all() -> Vec<Model> {
    let sheet_list: Vec<Model> = Vec::new();

    sheet_list
}

pub fn find_by_id(id: i32) -> Model {
    let temp: Model = null;
    temp
}

pub fn create_instance(req_body: String) -> () {
    let info: Model = serde_json::from_str(&req_body).expect("Failed to deserialize JSON");
    let new_instance = sheet_instance::ActiveModel {
        id: ActiveValue::Set(info.id),
        name: ActiveValue::Set(info.name),
        image_path: ActiveValue::Set(info.image_path),
        music_xml_path: ActiveValue::Set(info.music_xml_path),
        midi_path: ActiveValue::Set(info.music_xml_path),
    };
    let res = sheet_instance::insert(new_instance).exec(db).await?;
}

// pub fn put_instance(_id: i32, req_body: String) -> SheetInstance {
//     serde_json::from_str(&req_body).expect("Failed to deserialize JSON")
// }

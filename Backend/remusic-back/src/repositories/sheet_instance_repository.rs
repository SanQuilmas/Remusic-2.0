use crate::entities::{prelude::*, *};
use sea_orm::*;

// let mut temp =
//         SheetInstance::build_sheet_instance(id, "Yo".to_owned(), "image_path".to_owned());
//     temp._set_midi_path(String::from(
//         "https://magenta.github.io/magenta-js/music/demos/melody.mid",
//     ));
// /logo.webp
//     temp._set_musicxml_path(String::from("/MozaVeilSample.xml"));

pub async fn find_all(conn: DatabaseConnection) -> Vec<sheet_instance::Model> {
    let sheet_list: Vec<sheet_instance::Model> = SheetInstance::find()
        .all(&conn)
        .await
        .expect("Error Getting All Sheets");

    sheet_list
}

pub async fn find_by_id(id: i32, conn: DatabaseConnection) -> Option<sheet_instance::Model> {
    let found_sheet: Option<sheet_instance::Model> = SheetInstance::find_by_id(id)
        .one(&conn)
        .await
        .expect("Error getting sheet by id");
    found_sheet
}

pub async fn create_instance(req_body: String, conn: DatabaseConnection) -> sheet_instance::Model {
    let info: sheet_instance::Model =
        serde_json::from_str(&req_body).expect("Failed to deserialize JSON");

    let new_instance = sheet_instance::ActiveModel {
        id: ActiveValue::not_set(),
        name: ActiveValue::Set(info.name),
        image_path: ActiveValue::Set(info.image_path),
        music_xml_path: ActiveValue::Set(info.music_xml_path),
        midi_path: ActiveValue::Set(info.midi_path),
    };
    new_instance
        .insert(&conn)
        .await
        .expect("Failed to insert into Database")
}

// pub fn put_instance(_id: i32, req_body: String) -> SheetInstance {
//     serde_json::from_str(&req_body).expect("Failed to deserialize JSON")
// }

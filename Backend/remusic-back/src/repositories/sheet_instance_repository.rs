use crate::entities::dto_sheet_instance::DtoUpdateSheetInstance;
use crate::entities::{dto_sheet_instance::DtoCreateSheetInstance, prelude::*, *};
use crate::env_variables::{MACHINE_FOLDER_PATH, MACHINE_FOLDER_URL};
use actix_web::{HttpResponse, Responder};
use base64::{Engine, engine::general_purpose};
use sea_orm::*;
use std::fs;
use std::io::Write;

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
        .expect("Error Getting All Sheets")
        .into_iter()
        .collect();

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
    let info: DtoCreateSheetInstance =
        serde_json::from_str(&req_body).expect("Failed to deserialize JSON");

    let image_blob_bytes: Vec<u8> = general_purpose::STANDARD
        .decode(&info.image_blob)
        .expect("Failed to decode base64 image_blob");

    let folder_path = MACHINE_FOLDER_PATH.to_owned() + &info.name;
    let folder_url = format!("{}{}", MACHINE_FOLDER_URL, &info.name);

    fs::create_dir_all(&folder_path).expect("Failed to create directory");

    let image_path = format!("{}/{}_image.png", folder_path, info.id);
    let mut image_file = fs::File::create(&image_path).expect("Failed to create image file");
    image_file
        .write_all(&image_blob_bytes)
        .expect("Failed to write image data");

    let image_url = format!("{}/{}_image.png", &folder_url, info.id);

    let new_instance = sheet_instance::ActiveModel {
        id: ActiveValue::not_set(),
        name: ActiveValue::Set(info.name),
        image_path: ActiveValue::Set(image_url),
        music_xml_path: ActiveValue::NotSet,
        midi_path: ActiveValue::NotSet,
    };
    new_instance
        .insert(&conn)
        .await
        .expect("Failed to insert into Database")
}

pub async fn delete_by_id(id: i32, conn: DatabaseConnection) -> impl Responder {
    let deleted_entry = sheet_instance::ActiveModel {
        id: ActiveValue::Set(id),
        ..Default::default()
    };
    match deleted_entry.delete(&conn).await {
        Ok(delete_result) => {
            if delete_result.rows_affected > 0 {
                HttpResponse::Ok().body("Deleted successfully")
            } else {
                HttpResponse::NotFound().body("No entry found with that id")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn put_instance(
    id: i32,
    req_body: String,
    conn: DatabaseConnection,
) -> sheet_instance::Model {
    let info: DtoUpdateSheetInstance =
        serde_json::from_str(&req_body).expect("Failed to deserialize JSON");

    let music_xml_blob_bytes: Vec<u8> = general_purpose::STANDARD
        .decode(&info.music_xml_blob.unwrap())
        .expect("Failed to decode base64 music_xml_blob");

    let midi_blob_bytes: Vec<u8> = general_purpose::STANDARD
        .decode(&info.midi_blob.unwrap())
        .expect("Failed to decode base64 midi_blob");

    let old_instance = SheetInstance::find_by_id(id)
        .one(&conn)
        .await
        .expect("Error getting sheet by id")
        .unwrap();

    let folder_path = MACHINE_FOLDER_PATH.to_owned() + &old_instance.name;
    let folder_url = format!("{}{}", MACHINE_FOLDER_URL, &old_instance.name);

    let music_xml_path = format!("{}/{}_music_xml.xml", folder_path, info.id);
    let mut xml_file = fs::File::create(&music_xml_path).expect("Failed to create XML file");
    xml_file
        .write_all(&music_xml_blob_bytes)
        .expect("Failed to write image data");

    let music_xml_url = format!("{}/{}_music_xml.xml", &folder_url, info.id);

    let music_midi_path = format!("{}/{}_music_midi.mid", folder_path, info.id);
    let mut midi_file = fs::File::create(&music_midi_path).expect("Failed to create MIDI file");
    midi_file
        .write_all(&midi_blob_bytes)
        .expect("Failed to write image data");

    let music_midi_url = format!("{}/{}_music_midi.mid", &folder_url, info.id);

    let updated_instance = sheet_instance::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(old_instance.name),
        image_path: ActiveValue::Set(old_instance.image_path),
        music_xml_path: ActiveValue::Set(Some(music_xml_url)),
        midi_path: ActiveValue::Set(Some(music_midi_url)),
    };

    updated_instance
        .update(&conn)
        .await
        .expect("Failed to update sheet instance")
}

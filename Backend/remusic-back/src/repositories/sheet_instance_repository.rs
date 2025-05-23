use crate::entities::{
    dto_sheet_instance::{DtoCreateSheetInstance, DtoUpdateSheetInstance},
    prelude::*,
    *,
};
use actix_web::{HttpResponse, Responder};
use base64::{Engine, engine::general_purpose};
use sea_orm::*;

// let mut temp =
//         SheetInstance::build_sheet_instance(id, "Yo".to_owned(), "image_path".to_owned());
//     temp._set_midi_path(String::from(
//         "https://magenta.github.io/magenta-js/music/demos/melody.mid",
//     ));
// /logo.webp
//     temp._set_musicxml_path(String::from("/MozaVeilSample.xml"));

pub async fn find_all(conn: DatabaseConnection) -> Vec<DtoCreateSheetInstance> {
    let sheet_list: Vec<DtoCreateSheetInstance> = SheetInstance::find()
        .all(&conn)
        .await
        .expect("Error Getting All Sheets")
        .into_iter()
        .map(DtoCreateSheetInstance::from)
        .collect();

    sheet_list
}

pub async fn find_by_id(id: i32, conn: DatabaseConnection) -> Option<DtoCreateSheetInstance> {
    let found_sheet: Option<DtoCreateSheetInstance> = SheetInstance::find_by_id(id)
        .one(&conn)
        .await
        .expect("Error getting sheet by id")
        .map(DtoCreateSheetInstance::from);
    found_sheet
}

pub async fn create_instance(req_body: String, conn: DatabaseConnection) -> sheet_instance::Model {
    let info: DtoCreateSheetInstance =
        serde_json::from_str(&req_body).expect("Failed to deserialize JSON");

    let image_blob_bytes: Vec<u8> = general_purpose::STANDARD
        .decode(&info.image_blob)
        .expect("Failed to decode base64 image_blob");

    let new_instance = sheet_instance::ActiveModel {
        id: ActiveValue::not_set(),
        name: ActiveValue::Set(info.name),
        image_blob: ActiveValue::Set(image_blob_bytes),
        music_xml_blob: ActiveValue::not_set(),
        midi_blob: ActiveValue::not_set(),
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

    let updated_instance = sheet_instance::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(old_instance.name),
        image_blob: ActiveValue::Set(old_instance.image_blob),
        music_xml_blob: ActiveValue::Set(Some(music_xml_blob_bytes)),
        midi_blob: ActiveValue::Set(Some(midi_blob_bytes)),
    };

    updated_instance
        .update(&conn)
        .await
        .expect("Failed to update sheet instance")
}

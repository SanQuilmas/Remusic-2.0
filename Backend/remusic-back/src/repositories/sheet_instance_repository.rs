use crate::entities::dto_sheet_instance::DtoUpdateSheetInstance;
use crate::entities::{dto_sheet_instance::DtoCreateSheetInstance, prelude::*, *};
use crate::env_variables::{
    KAFKA_ALREADY_UPDATED_TOPIC, KAFKA_NOT_UPDATED_TOPIC, MACHINE_FOLDER_PATH, MACHINE_FOLDER_URL,
};
use crate::utilities::bytes_manipulation::decode_blob;
use crate::utilities::kafka_producer::kafka_produce_message;
use actix_web::{HttpResponse, Responder};
use sea_orm::*;
use std::fs;
use std::io::Write;

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

    let new_instance = sheet_instance::ActiveModel {
        id: ActiveValue::not_set(),
        name: ActiveValue::Set(info.name),
        image_path: ActiveValue::NotSet,
        music_xml_path: ActiveValue::NotSet,
        midi_path: ActiveValue::NotSet,
    };

    let inserted_instance = new_instance
        .insert(&conn)
        .await
        .expect("Failed to insert into Database");

    let image_blob_bytes: Vec<u8> = decode_blob(&info.image_blob);

    let folder_path = format!(
        "{}{}_{}",
        MACHINE_FOLDER_PATH, &inserted_instance.id, &inserted_instance.name
    );
    let folder_url = format!(
        "{}{}_{}",
        MACHINE_FOLDER_URL, &inserted_instance.id, &inserted_instance.name
    );

    fs::create_dir_all(&folder_path).expect("Failed to create directory");

    let image_path = format!("{}/{}_image.png", folder_path, inserted_instance.id);
    let mut image_file = fs::File::create(&image_path).expect("Failed to create image file");
    image_file
        .write_all(&image_blob_bytes)
        .expect("Failed to write image data");

    let image_url = format!("{}/{}_image.png", &folder_url, inserted_instance.id);

    let new_instance = sheet_instance::ActiveModel {
        id: ActiveValue::Set(inserted_instance.id),
        name: ActiveValue::Set(inserted_instance.name),
        image_path: ActiveValue::Set(Some(image_url)),
        music_xml_path: ActiveValue::NotSet,
        midi_path: ActiveValue::NotSet,
    };

    let inserted_instance = new_instance
        .update(&conn)
        .await
        .expect("Failed to update sheet instance");

    let message_payload =
        serde_json::to_string(&inserted_instance).expect("Failed to serialize inserted instance");

    if let Err(e) = kafka_produce_message(
        KAFKA_NOT_UPDATED_TOPIC,
        &message_payload,
        inserted_instance.id,
    )
    .await
    {
        eprintln!("Kafka error: {}", e);
    }

    inserted_instance
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

    let music_xml_blob_bytes: Vec<u8> = decode_blob(&info.music_xml_blob.unwrap());

    let midi_blob_bytes: Vec<u8> = decode_blob(&info.midi_blob.unwrap());

    let old_instance = SheetInstance::find_by_id(id)
        .one(&conn)
        .await
        .expect("Error getting sheet by id")
        .unwrap();

    let folder_path = format!(
        "{}{}_{}",
        MACHINE_FOLDER_PATH, &old_instance.id, &old_instance.name
    );
    let folder_url = format!(
        "{}{}_{}",
        MACHINE_FOLDER_URL, &old_instance.id, &old_instance.name
    );

    let music_xml_path = format!("{}/{}_music_xml.xml", folder_path, old_instance.id);
    let mut xml_file = fs::File::create(&music_xml_path).expect("Failed to create XML file");
    xml_file
        .write_all(&music_xml_blob_bytes)
        .expect("Failed to write image data");

    let music_xml_url = format!("{}/{}_music_xml.xml", &folder_url, old_instance.id);

    let music_midi_path = format!("{}/{}_music_midi.mid", folder_path, old_instance.id);
    let mut midi_file = fs::File::create(&music_midi_path).expect("Failed to create MIDI file");
    midi_file
        .write_all(&midi_blob_bytes)
        .expect("Failed to write image data");

    let music_midi_url = format!("{}/{}_music_midi.mid", &folder_url, old_instance.id);

    let updated_instance = sheet_instance::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(old_instance.name),
        image_path: ActiveValue::Set(old_instance.image_path),
        music_xml_path: ActiveValue::Set(Some(music_xml_url)),
        midi_path: ActiveValue::Set(Some(music_midi_url)),
    };

    let uptodate_instance = updated_instance
        .update(&conn)
        .await
        .expect("Failed to update sheet instance");

    let message_payload =
        serde_json::to_string(&uptodate_instance).expect("Failed to serialize updated instance");

    if let Err(e) = kafka_produce_message(
        KAFKA_ALREADY_UPDATED_TOPIC,
        &message_payload,
        uptodate_instance.id,
    )
    .await
    {
        eprintln!("Kafka error: {}", e);
    }

    uptodate_instance
}

use std::collections::HashMap;

use actix_web::{HttpResponse, Responder};
use sea_orm::*;

use crate::{
    entities::sheet_instance::{self},
    repositories::{
        self,
        sheet_instance_repository::{delete_by_id, find_all, find_by_id},
    },
    utilities::kafka_consumer::kafka_consume_messages,
};

pub async fn get_all_sheets(conn: DatabaseConnection) -> impl Responder {
    let sheet_list: Vec<sheet_instance::Model> = find_all(conn.clone()).await;
    HttpResponse::Ok().json(sheet_list)
}

pub async fn get_sheet_instance(id: i32, conn: DatabaseConnection) -> impl Responder {
    match find_by_id(id, conn.clone()).await {
        Some(sheet_instance) => HttpResponse::Ok().json(sheet_instance),
        None => HttpResponse::NotFound().body("Sheet instance not found"),
    }
}

pub async fn create_sheet_instance(req_body: String, conn: DatabaseConnection) -> impl Responder {
    let sheet_instance: sheet_instance::Model =
        repositories::sheet_instance_repository::create_instance(req_body, conn.clone()).await;
    HttpResponse::Ok().json(sheet_instance)
}

pub async fn delete_sheet_instance(id: i32, conn: DatabaseConnection) -> impl Responder {
    delete_by_id(id, conn.clone()).await
}

pub async fn put_sheet_instance(
    id: i32,
    req_body: String,
    conn: DatabaseConnection,
) -> impl Responder {
    let sheet_instance: sheet_instance::Model =
        repositories::sheet_instance_repository::put_instance(id, req_body, conn.clone()).await;
    HttpResponse::Ok().json(sheet_instance)
}

pub async fn get_kafka_by_name(topic: String) -> impl Responder {
    let keys_map: HashMap<i32, String> = kafka_consume_messages(&topic).await;
    HttpResponse::Ok().json(keys_map)
}

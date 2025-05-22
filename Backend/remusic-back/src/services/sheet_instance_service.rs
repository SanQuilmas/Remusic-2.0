use actix_web::{HttpResponse, Responder};
use sea_orm::*;

use crate::{
    entities::{
        dto_sheet_instance::DtoCreateSheetInstance,
        sheet_instance::{self},
    },
    repositories::{
        self,
        sheet_instance_repository::{delete_by_id, find_all, find_by_id},
    },
};

pub async fn get_all_sheets(conn: DatabaseConnection) -> impl Responder {
    let sheet_list: Vec<DtoCreateSheetInstance> = find_all(conn.clone()).await;
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

pub async fn patch_sheet_instance(id: i32, req_body: String, conn: DatabaseConnection) -> impl Responder {
    let sheet_instance: sheet_instance::Model =
        repositories::sheet_instance_repository::patch_instance(id, req_body, conn.clone()).await;
    HttpResponse::Ok().json(sheet_instance)
}

use actix_web::{Responder, /*put,*/ delete, get, post, web};
use log::info;
use sea_orm::*;

use crate::services::sheet_instance_service::{create_sheet_instance, get_all_sheets};

#[get("/sheet")]
pub async fn get_sheets(conn: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Get Sheets Service Called");
    get_all_sheets(conn.get_ref().clone()).await
}

#[post("/sheet")]
pub async fn post_sheet(req_body: String, conn: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Post Sheet Service Called");
    create_sheet_instance(req_body, conn.get_ref().clone()).await
}

#[get("/sheet/{id}")]
pub async fn get_sheet(
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> impl Responder {
    info!("Get Sheet by ID Service Called");
    let id = path.into_inner();
    crate::services::sheet_instance_service::get_sheet_instance(id, conn.get_ref().clone()).await
}

#[delete("/sheet/{id}")]
pub async fn delete_sheet(
    path: web::Path<i32>,
    conn: web::Data<DatabaseConnection>,
) -> impl Responder {
    info!("Delete Sheet by ID Service Called");
    let id = path.into_inner();
    crate::services::sheet_instance_service::delete_sheet_instance(id, conn.get_ref().clone()).await
}

// #[put("/sheet/{id}")]
// pub async fn put_sheet(path: web::Path<i32>, req_body: String, conn: web::Data<DatabaseConnection>) -> impl Responder {
//     let id = path.into_inner();
//     crate::services::sheet_instance_service::put_sheet_instance(id, req_body).await
// }

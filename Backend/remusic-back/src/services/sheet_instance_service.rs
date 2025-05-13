// use actix_web::{HttpResponse, Responder};

// use crate::{entities::sheet_instance::Model as SheetInstance, repositories};

// pub async fn get_all_sheets() -> impl Responder {
//     let sheet_list: Vec<SheetInstance> = repositories::sheet_instance_repository::find_all();
//     HttpResponse::Ok().json(sheet_list)
// }

// pub async fn get_sheet_instance(id: i32) -> impl Responder {
//     let sheet_instance: SheetInstance = repositories::sheet_instance_repository::find_by_id(id);
//     HttpResponse::Ok().json(sheet_instance)
// }

// // pub async fn create_sheet_instance(req_body: String) -> impl Responder {
// //     let sheet_instance: SheetInstance =
// //         repositories::sheet_instance_repository::create_instance(req_body);
// //     HttpResponse::Ok().json(sheet_instance)
// // }

// pub async fn delete_sheet_instance(id: i32) -> impl Responder {
//     HttpResponse::Ok().body(format!("ID: {}", id))
// }

// pub async fn put_sheet_instance(id: i32, req_body: String) -> impl Responder {
//     let sheet_instance: SheetInstance =
//         repositories::sheet_instance_repository::put_instance(id, req_body);
//     HttpResponse::Ok().json(sheet_instance)
// }

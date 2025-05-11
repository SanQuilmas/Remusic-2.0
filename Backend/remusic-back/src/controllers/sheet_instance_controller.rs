use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

#[get("/sheet")]
pub async fn get_sheets() -> impl Responder {
    crate::services::sheet_instance_service::get_all_sheets().await
}

#[post("/sheet")]
pub async fn post_sheet(req_body: String) -> impl Responder {
    crate::services::sheet_instance_service::create_sheet_instance(req_body).await
}

#[get("/sheet/{id}")]
pub async fn get_sheet(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    crate::services::sheet_instance_service::get_sheet_instance(id).await
}

#[delete("/sheet/{id}")]
pub async fn delete_sheet(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    crate::services::sheet_instance_service::delete_sheet_instance(id).await
}

#[put("/sheet/{id}")]
pub async fn put_sheet(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    crate::services::sheet_instance_service::put_sheet_instance(id).await
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

use actix_web::{HttpResponse, Responder, delete, get, post, web};

#[get("/sheet")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("All")
}

#[get("/sheet/{id}")]
pub async fn get_by_id(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("ID: {}", id))
}

#[delete("/sheet/{id}")]
pub async fn delete(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Deleted ID: {}", id))
}

#[post("/sheet")]
pub async fn create(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

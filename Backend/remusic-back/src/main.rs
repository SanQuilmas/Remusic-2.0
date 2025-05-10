use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

/*
    controllers:
        Place all the route handlers here:

    services:
        Business logic or complex operations should go here.

    entities:
        This would contain your domain models.
        If you have any structs representing data models or database entities, they go here.

    repositories:
        Any database interaction or persistence logic goes here.
        If you want to read/write data, add functions like get_user, create_user, etc.
*/

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

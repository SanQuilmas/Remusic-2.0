use actix_web::{App, HttpServer, web};
use migrator::Migrator;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::*;

mod controllers;
mod entities;
mod repositories;
mod services;

mod migrator;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const DATABASE_URL: &str = "postgresql://user:password@localhost:6432/postgres";
    let db: DatabaseConnection = Database::connect(DATABASE_URL)
        .await
        .expect("Failed to connect to database");

    let schema_manager = SchemaManager::new(&db);
    Migrator::refresh(&db).await.expect("Migrator Failed");
    assert!(
        schema_manager
            .has_table("sheet_instance")
            .await
            .expect("Failed to place table")
    );

    HttpServer::new(move || {
        App::new()
            // .service(controllers::sheet_instance_controller::get_sheets)
            // .service(controllers::sheet_instance_controller::get_sheet)
            // // .service(controllers::sheet_instance_controller::post_sheet)
            // // .service(controllers::sheet_instance_controller::put_sheet)
            // .service(controllers::sheet_instance_controller::delete_sheet)
            .app_data(web::Data::new(db.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use env_logger::Env;
use env_variables::{DATABASE_URL, IP_BIND, STATIC_FOLDER};
use migrator::Migrator;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::*;

mod controllers;
mod entities;
mod repositories;
mod services;

mod env_variables;
mod migrator;
mod utilities;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::new(db.clone()))
            .service(actix_files::Files::new("/static", STATIC_FOLDER).show_files_listing())
            .service(controllers::sheet_instance_controller::get_sheets)
            .service(controllers::sheet_instance_controller::post_sheet)
            .service(controllers::sheet_instance_controller::get_sheet)
            .service(controllers::sheet_instance_controller::delete_sheet)
            .service(controllers::sheet_instance_controller::put_sheet)
            .service(controllers::sheet_instance_controller::get_kafka_keys)
    })
    .bind((IP_BIND, 8080))?
    .run()
    .await
}

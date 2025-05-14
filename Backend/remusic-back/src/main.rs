use actix_web::{App, HttpServer, web};
use controllers::sheet_instance_controller::{get_sheets, post_sheet};
use env_logger::Env;
use migrator::Migrator;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::*;

mod controllers;
mod entities;
mod repositories;
mod services;

mod migrator;

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

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(get_sheets)
            .service(post_sheet)
            .service(controllers::sheet_instance_controller::get_sheet)
        // .service(controllers::sheet_instance_controller::put_sheet)
        // .service(controllers::sheet_instance_controller::delete_sheet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

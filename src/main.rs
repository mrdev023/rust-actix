#[macro_use]
extern crate diesel; // Required for schema.rs

pub(self) mod controllers;
pub(self) mod models;
pub(self) mod schema;
pub(self) mod views;

use actix_web::{middleware, web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(controllers::init_routes)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

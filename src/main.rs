pub(self) mod controllers;
pub(self) mod views;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(web::scope("/hello").service(controllers::hello_controller::index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

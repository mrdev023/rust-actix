use actix_web::{get, web, HttpResponse, Responder};
use yew::ServerRenderer;

use crate::views::components::hello::{Hello, HelloProps};

#[get("/{name}")]
async fn index(name: web::Path<String>) -> impl Responder {
    let name = name.into_inner();
    let renderer = ServerRenderer::<Hello>::with_props(HelloProps { name });
    HttpResponse::Ok().body(renderer.render().await)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/hello").service(index));
}


use actix_web::{get, Responder, web, HttpResponse};
use yew::ServerRenderer;

use crate::front::components::hello::{Hello, HelloProps};

#[get("/{name}")]
async fn index(name: web::Path<String>) -> impl Responder {
    let name = name.into_inner();
    let renderer = ServerRenderer::<Hello>::with_props(HelloProps {
        name
    });
    HttpResponse::Ok().body(renderer.render().await)
}
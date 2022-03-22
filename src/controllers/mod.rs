use actix_web::web;

mod hello_controller;
mod users_controller;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.configure(hello_controller::init_routes);
  cfg.configure(users_controller::init_routes);
}
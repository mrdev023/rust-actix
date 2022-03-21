use actix_web::{get, web, Error, HttpResponse};
use yew::ServerRenderer;

use crate::{
    models::user::{NewUser, User},
    views::components::{
        user::{UserComponent, UserComponentProps},
        users::{UsersComponent, UsersComponentProps},
    },
    DbPool, controllers::run_query,
};

#[get("")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = run_query(|conn| User::all(conn), &pool).await?;

    let renderer = ServerRenderer::<UsersComponent>::with_props(UsersComponentProps {
        users: users.clone(),
    });
    Ok(HttpResponse::Ok().body(renderer.render().await))
}

#[get("create")]
async fn create(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let user = User::new(NewUser {
        name: "John".to_string(),
    });

    let u = user.clone();
    run_query(|conn| u.insert(conn), &pool).await?;

    let renderer =
        ServerRenderer::<UserComponent>::with_props(UserComponentProps { user: user.clone() });
    Ok(HttpResponse::Ok().body(renderer.render().await))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(index).service(create));
}

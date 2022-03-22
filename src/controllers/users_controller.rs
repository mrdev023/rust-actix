use actix_web::{get, web, Error, HttpResponse};
use yew::ServerRenderer;

use crate::{
    models::{
        user::{
            NewUser,
            User,
        },
        run_query,
        DbPool
    },
    views::components::{
        user::{UserComponent, UserComponentProps},
        users::{UsersComponent, UsersComponentProps},
    },
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
    let user = run_query(|conn| {
        let user = User::new(NewUser {
            name: "John".to_string(),
        });
        user.insert(conn)
    }, &pool).await?;

    let renderer =
        ServerRenderer::<UserComponent>::with_props(UserComponentProps { user: user });
    Ok(HttpResponse::Ok().body(renderer.render().await))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(index).service(create));
}

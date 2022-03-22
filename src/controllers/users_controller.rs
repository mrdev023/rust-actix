use actix_web::{get, post, web, Error, HttpResponse, delete, patch};
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

#[get("/{id}")]
async fn show(pool: web::Data<DbPool>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let user = run_query(move |conn| User::find_by_id(conn, &id), &pool).await?;

    let renderer =
        ServerRenderer::<UserComponent>::with_props(UserComponentProps { user: user });
    Ok(HttpResponse::Ok().body(renderer.render().await))
}

#[post("")]
async fn create(pool: web::Data<DbPool>, user: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    let user = run_query(move |conn| User::create(conn, &user), &pool).await?;

    let renderer =
        ServerRenderer::<UserComponent>::with_props(UserComponentProps { user: user });
    Ok(HttpResponse::Ok().body(renderer.render().await))
}

#[patch("/{id}")]
async fn update(pool: web::Data<DbPool>, id: web::Path<String>, user: web::Json<User>) -> Result<HttpResponse, Error> {
    let user = run_query(move |conn| User::find_by_id(conn, &id)?.update(conn, &user), &pool).await?;

    let renderer =
        ServerRenderer::<UserComponent>::with_props(UserComponentProps { user: user });
    Ok(HttpResponse::Ok().body(renderer.render().await))
}

#[delete("/{id}")]
async fn delete(pool: web::Data<DbPool>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let user = run_query(move |conn| User::find_by_id(conn, &id)?.delete(conn), &pool).await?;

    let renderer = ServerRenderer::<UserComponent>::with_props(UserComponentProps { user: user });
    Ok(HttpResponse::Ok().body(renderer.render().await))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
        .service(index)
        .service(show)
        .service(create)
        .service(update)
        .service(delete)
    );
}

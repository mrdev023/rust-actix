use actix_web::{get, web, Error, HttpResponse};
use yew::ServerRenderer;

use crate::{
    models::user::{NewUser, User},
    views::components::{
        user::{UserComponent, UserComponentProps},
        users::{UsersComponent, UsersComponentProps}
    },
    DbPool,
};

#[get("")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = pool.get()?;
        User::all(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let renderer =
        ServerRenderer::<UsersComponent>::with_props(UsersComponentProps { users: users.clone() });
    Ok(HttpResponse::Ok().body(renderer.render().await))
}

#[get("")]
async fn create(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let user = User::new(NewUser {
        name: "John".to_string(),
    });

    let u = user.clone();
    web::block(move || {
        let conn = pool.get()?;
        u.insert(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let renderer =
        ServerRenderer::<UserComponent>::with_props(UserComponentProps { user: user.clone() });
    Ok(HttpResponse::Ok().body(renderer.render().await))
}

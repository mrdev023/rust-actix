use actix_web::{web, Error};
use diesel::r2d2;

use crate::{DbPool};

mod hello_controller;
mod users_controller;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub(self) async fn run_query<T, F>(query: F, pool: &web::Data<DbPool>) -> Result<T, Error> 
where
    F: Fn(&diesel::SqliteConnection) -> diesel::QueryResult<T>
{
  web::block(move || {
      let conn = pool.get()?;
      query(&conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.configure(hello_controller::init_routes);
  cfg.configure(users_controller::init_routes);
}
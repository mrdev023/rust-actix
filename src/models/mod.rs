use actix_web::{web, Error};
use diesel::{r2d2::{self, ConnectionManager}, SqliteConnection};

pub mod user;

pub(crate) type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub(crate) async fn run_query<T, F>(query: F, pool: &web::Data<DbPool>) -> Result<T, Error> 
where
    F: Fn(&diesel::SqliteConnection) -> diesel::QueryResult<T> + Send + 'static,
    T: Send + 'static
{
  let conn = pool.get()
    .map_err(actix_web::error::ErrorInternalServerError)?;
  web::block(move || query(&conn) )
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)
}
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

use super::DbError;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

impl User {
    pub fn new(new_user: NewUser) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: new_user.name,
        }
    }

    pub fn all(conn: &SqliteConnection) -> Result<Vec<Self>, DbError> {
        users::table.load::<Self>(conn).map_err(Into::into)
    }

    pub fn find_by_id(conn: &SqliteConnection, id: String) -> Result<Self, DbError> {
        users::table
            .filter(users::id.eq(id))
            .first::<Self>(conn)
            .map_err(Into::into)
    }

    pub fn insert(&self, conn: &SqliteConnection) -> Result<(), DbError> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users).values(self).execute(conn)?;

        Ok(())
    }

    pub fn update(&self, conn: &SqliteConnection) -> Result<(), DbError> {
        use crate::schema::users::dsl::*;

        diesel::update(users.find(self.id.clone()))
            .set(self)
            .execute(conn)?;

        Ok(())
    }

    pub fn delete(&self, conn: &SqliteConnection) -> Result<(), DbError> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(self.id.clone()))).execute(conn)?;

        Ok(())
    }
}

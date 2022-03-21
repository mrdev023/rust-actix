use diesel::{prelude::*, result::QueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

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

    pub fn all(conn: &SqliteConnection) -> QueryResult<Vec<Self>> {
        users::table.load(conn)
    }

    pub fn find_by_id(conn: &SqliteConnection, id: String) -> QueryResult<Self> {
        users::table
            .filter(users::id.eq(id))
            .first(conn)
    }

    pub fn insert(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users).values(self).execute(conn)
    }

    pub fn update(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::update(users.find(self.id.clone()))
            .set(self)
            .execute(conn)
    }

    pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(self.id.clone()))).execute(conn)
    }
}

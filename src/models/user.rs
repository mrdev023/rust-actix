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

    pub fn find_by_id(conn: &SqliteConnection, id: &String) -> QueryResult<Self> {
        users::table
            .filter(users::id.eq(id))
            .first(conn)
    }

    pub fn create(conn: &SqliteConnection, new_user: &NewUser) -> QueryResult<Self> {
        let user = User::new(new_user.clone());
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)?;

        Ok(user)
    }

    pub fn update(&self, conn: &SqliteConnection, data: &User) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        diesel::update(users.find(self.id.clone()))
            .set(data)
            .execute(conn)?;

        Ok(data.clone())
    }

    pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(self.id.clone()))).execute(conn)?;

        Ok(self.clone())
    }
}

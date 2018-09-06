use diesel::prelude::*;
use failure::Fallible;

use super::conn::Conn;
use super::schema::users;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn verify(&self, password: &str) -> bool {
        bcrypt::verify(&password, &self.password).unwrap_or(false)
    }

    pub fn create(
        conn: &Conn,
        username: String,
        email: String,
        password: String,
    ) -> Fallible<User> {
        let new_user = NewUser {
            username,
            email,
            password: bcrypt::hash(&password, bcrypt::DEFAULT_COST)?,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn.get())
            .map_err(Into::into)
    }

    pub fn find_by_email(conn: &Conn, email: String) -> Fallible<Option<User>> {
        use super::schema::users::dsl;
        dsl::users
            .filter(dsl::email.eq(email))
            .get_result(conn.get())
            .optional()
            .map_err(Into::into)
    }

    pub fn find_by_id(conn: &Conn, id: i32) -> Fallible<Option<User>> {
        use super::schema::users::dsl;
        dsl::users
            .filter(dsl::id.eq(id))
            .get_result(conn.get())
            .optional()
            .map_err(Into::into)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

use std::fmt;

use futures::compat::Future01CompatExt;
use futures::future::{Future, TryFutureExt};

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use bcrypt;
use failure::Fallible;
use tokio::prelude::future::poll_fn as poll_fn_01;
use tokio::prelude::Future as _Future01;
use tokio_threadpool::blocking;

use super::model::{NewUser, User};
use super::schema::users;

pub struct ConnPool {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnPool {
    pub fn init(url: impl Into<String>) -> Fallible<ConnPool> {
        let manager = ConnectionManager::<PgConnection>::new(url.into());
        let pool = Pool::builder().max_size(15).build(manager)?;
        Ok(ConnPool { pool })
    }

    pub fn acquire_connection(&self) -> impl Future<Output = Fallible<Conn>> {
        let pool = self.pool.clone();
        poll_fn_01(move || blocking(|| pool.get()))
            .then(|result| match result {
                Ok(Ok(conn)) => Ok(Conn { conn }),
                Ok(Err(err)) => Err(err.into()),
                Err(err) => Err(err.into()),
            }).compat()
            .into_future()
    }
}

pub struct Conn {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl fmt::Debug for Conn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Conn").finish()
    }
}

impl Conn {
    pub fn create_user(&self, username: String, email: String, password: String) -> Fallible<User> {
        let new_user = NewUser {
            username,
            email,
            password: bcrypt::hash(&password, bcrypt::DEFAULT_COST)?,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&*self.conn)
            .map_err(Into::into)
    }

    pub fn get_user_by_email(&self, email: String) -> Fallible<Option<User>> {
        use super::schema::users::dsl;
        dsl::users
            .filter(dsl::email.eq(email))
            .get_result::<User>(&*self.conn)
            .optional()
            .map_err(Into::into)
    }

    pub fn find_user_by_id(&self, id: i32) -> Fallible<Option<User>> {
        use super::schema::users::dsl;
        dsl::users
            .filter(dsl::id.eq(id))
            .get_result::<User>(&*self.conn)
            .optional()
            .map_err(Into::into)
    }
}

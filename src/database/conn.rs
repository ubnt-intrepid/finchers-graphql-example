use std::fmt;

use futures::compat::Future01CompatExt;
use futures::future::{Future, TryFutureExt};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use failure::Fallible;
use tokio::prelude::future::poll_fn as poll_fn_01;
use tokio::prelude::Future as _Future01;
use tokio_threadpool::blocking;

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
    #[inline]
    pub fn get(&self) -> &PgConnection {
        &*self.conn
    }
}

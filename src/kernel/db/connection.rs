use crate::kernel::config::DATABASE_URL;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use std::sync::LazyLock;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct Db;

static POOL: LazyLock<DbPool> = LazyLock::new(|| {
    let manager = ConnectionManager::<PgConnection>::new(&*DATABASE_URL);

    Pool::builder()
        .min_idle(Some(5))
        .max_size(20)
        .build(manager)
        .expect("failed to create DB pool")
});

impl Db {
    pub fn get_pool() -> DbPool {
        POOL.clone()
    }

    pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        Self::get_pool().get().expect("Failed to get DB connection")
    }
}

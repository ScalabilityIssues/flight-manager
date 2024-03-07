use std::ops::DerefMut;

use sqlx::{migrate::Migrator, PgPool, Postgres};

mod error;
pub use error::DatabaseError;

pub const MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone)]
pub struct Database(sqlx::PgPool);

impl Database {
    #[inline]
    pub fn from_pool(pool: PgPool) -> Self {
        Self(pool)
    }

    #[inline]
    pub async fn begin(&self) -> Result<Transaction<'_>, DatabaseError> {
        let t = self.0.begin().await?;
        Ok(Transaction(t))
    }
}

pub struct Transaction<'c>(sqlx::Transaction<'c, Postgres>);

impl Transaction<'_> {
    #[inline]
    pub async fn commit(self) -> Result<(), DatabaseError> {
        self.0.commit().await?;
        Ok(())
    }

    #[inline]
    pub async fn rollback(self) -> Result<(), DatabaseError> {
        self.0.rollback().await?;
        Ok(())
    }

    #[inline]
    pub fn get_conn(&mut self) -> &mut sqlx::PgConnection {
        self.0.deref_mut()
    }
}

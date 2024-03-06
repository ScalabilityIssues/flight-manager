use std::ops::DerefMut;

use sqlx::{migrate::Migrator, postgres::PgQueryResult, PgPool, Postgres};

pub const MIGRATOR: Migrator = sqlx::migrate!();

pub enum QueryError {
    NotFound,
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
    Unexpected(&'static str),
}

impl From<sqlx::Error> for QueryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => QueryError::NotFound,
            _ => QueryError::Other(Box::new(err)),
        }
    }
}

impl From<QueryError> for tonic::Status {
    fn from(err: QueryError) -> Self {
        match err {
            QueryError::NotFound => tonic::Status::not_found("could not find specified resource"),
            QueryError::Other(err) => tonic::Status::from_error(err),
            QueryError::Unexpected(msg) => tonic::Status::internal(msg),
        }
    }
}

impl QueryError {
    pub fn ensure_single_affected(res: PgQueryResult) -> Result<(), QueryError> {
        match res.rows_affected() {
            0 => Err(QueryError::NotFound),
            1 => Ok(()),
            _ => Err(QueryError::Unexpected("unexpected number of rows affected")),
        }
    }
}

#[derive(Clone)]
pub struct Database(sqlx::PgPool);

impl Database {
    pub fn from_pool(pool: PgPool) -> Self {
        Self(pool)
    }

    pub async fn begin(&self) -> Result<Transaction<'_>, QueryError> {
        let t = self.0.begin().await?;
        Ok(Transaction(t))
    }
}

pub struct Transaction<'c>(sqlx::Transaction<'c, Postgres>);

impl<'c> Transaction<'c> {
    pub async fn commit(self) -> Result<(), QueryError> {
        self.0.commit().await?;
        Ok(())
    }

    pub async fn rollback(self) -> Result<(), QueryError> {
        self.0.rollback().await?;
        Ok(())
    }

    pub fn get_conn(&mut self) -> &mut sqlx::PgConnection {
        self.0.deref_mut()
    }
}

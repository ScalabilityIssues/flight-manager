use sqlx::postgres::PgQueryResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("not found")]
    NotFound,
    #[error("error interacting with database: {0}")]
    Other(sqlx::Error),
    #[error("unexpected error querying database: {0}")]
    Unexpected(&'static str),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DatabaseError::NotFound,
            _ => DatabaseError::Other(err),
        }
    }
}

impl From<DatabaseError> for tonic::Status {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::NotFound => {
                tonic::Status::not_found("could not find specified resource")
            }
            _ => {
                tracing::error!(%error, "database error");
                tonic::Status::internal("database error")
            }
        }
    }
}

impl DatabaseError {
    pub fn ensure_single_affected(res: PgQueryResult) -> Result<(), DatabaseError> {
        match res.rows_affected() {
            0 => Err(DatabaseError::NotFound),
            1 => Ok(()),
            _ => Err(DatabaseError::Unexpected(
                "unexpected number of rows affected",
            )),
        }
    }
}

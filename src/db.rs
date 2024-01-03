use sqlx::postgres::PgQueryResult;

pub enum QueryError {
    NotFound,
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
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
            QueryError::NotFound => tonic::Status::not_found("plane"),
            QueryError::Other(err) => tonic::Status::from_error(err),
        }
    }
}

impl QueryError {
    pub fn ensure_single_affected(res: PgQueryResult) -> Result<(), QueryError> {
        match res.rows_affected() {
            0 => Err(QueryError::NotFound),
            1 => Ok(()),
            _ => panic!("unexpected number of rows affected"),
        }
    }
}

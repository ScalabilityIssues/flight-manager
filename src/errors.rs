use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("rabbitmq error: {0}")]
    RabbitError(#[from] amqprs::error::Error),
}

impl From<ApplicationError> for tonic::Status {
    fn from(error: ApplicationError) -> Self {
        match error {
            _ => {
                tracing::error!(%error, "internal error");
                let mut s = tonic::Status::internal("internal error");
                s.set_source(Arc::new(error));
                s
            }
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("collection not found")]
    CollectionNotFound,

    #[error("object not found")]
    ObjectNotFound,

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

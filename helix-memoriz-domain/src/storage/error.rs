use thiserror::Error;

//Define the possible errors
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Creation impossible")]
    CreationImpossible,
    #[error("Another error")]
    AnotherError,
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("Serde Json error: {source}")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },
    #[error("Postgres error: {source}")]
    PostGres {
        #[from]
        source: tokio_postgres::Error,
    },
    #[error("Sled error: {source}")]
    Sled {
        #[from]
        source: sled::Error,
    },
}

//Define the possible errors
#[derive(Error, Debug)]
pub enum SearchEngineError {
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Meilisearch error: {source}")]
    Sled {
        #[from]
        source: meilisearch_sdk::errors::Error,
    },
}

pub type StorageResult<T> = std::result::Result<T, StorageError>;
pub type SearchEngineResult<T> = std::result::Result<T, SearchEngineError>;

use std::{error::Error, fmt::Display, io};
use deadpool::managed::PoolError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum CreateFileError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
    FileSystem(io::Error),
}

impl Error for CreateFileError { }

impl Display for CreateFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateFileError::Sqlx(e) => e.fmt(f),
            CreateFileError::Pool(e) => e.fmt(f),
            CreateFileError::FileSystem(e) => write!(f, "CreateFileError::FileSystem({})", e)
        }
    }
}

impl From<SqlxError> for CreateFileError {
    fn from(value: SqlxError) -> Self {
        Self::Sqlx(value)
    }
}

impl From<PoolError<SqlxError>> for CreateFileError {
    fn from(value: PoolError<SqlxError>) -> Self {
        Self::Pool(value)
    }
}

impl From<io::Error> for CreateFileError {
    fn from(value: io::Error) -> Self {
        Self::FileSystem(value)
    }
}

#[derive(Debug)]
pub enum GetFileError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
    FileSystem(io::Error),
}

impl Error for GetFileError { }

impl Display for GetFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetFileError::Sqlx(e) => e.fmt(f),
            GetFileError::Pool(e) => e.fmt(f),
            GetFileError::FileSystem(e) => e.fmt(f)
        }
    }
}

impl From<SqlxError> for GetFileError {
    fn from(value: SqlxError) -> Self {
        Self::Sqlx(value)
    }
}

impl From<PoolError<SqlxError>> for GetFileError {
    fn from(value: PoolError<SqlxError>) -> Self {
        Self::Pool(value)
    }
}

impl From<io::Error> for GetFileError {
    fn from(value: io::Error) -> Self {
        Self::FileSystem(value)
    }
}
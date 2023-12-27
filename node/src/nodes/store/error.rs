use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use sqlx::Error as SqlxError;
use crate::nodes::node::ParsePgNodeError;

#[derive(Debug)]
pub enum CreateNodeError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
    Parse(ParsePgNodeError),
}

impl Error for CreateNodeError { }

impl Display for CreateNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlx(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f),
            Self::Parse(p) => p.fmt(f)
        }
    }
}

impl From<SqlxError> for CreateNodeError {
    fn from(value: SqlxError) -> Self {
        Self::Sqlx(value)
    }
}

impl From<PoolError<SqlxError>> for CreateNodeError {
    fn from(value: PoolError<SqlxError>) -> Self {
        Self::Pool(value)
    }
}

impl From<ParsePgNodeError> for CreateNodeError {
    fn from(value: ParsePgNodeError) -> Self {
        Self::Parse(value)
    }
}

#[derive(Debug)]
pub enum GetNodeError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
    Parse(ParsePgNodeError),
}

impl Error for GetNodeError { }

impl Display for GetNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlx(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f),
            Self::Parse(p) => p.fmt(f)
        }
    }
}

impl From<SqlxError> for GetNodeError {
    fn from(value: SqlxError) -> Self {
        Self::Sqlx(value)
    }
}

impl From<PoolError<SqlxError>> for GetNodeError {
    fn from(value: PoolError<SqlxError>) -> Self {
        Self::Pool(value)
    }
}

impl From<ParsePgNodeError> for GetNodeError {
    fn from(value: ParsePgNodeError) -> Self {
        Self::Parse(value)
    }
}

#[derive(Debug)]
pub enum DeleteNodeError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for DeleteNodeError { }

impl Display for DeleteNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlx(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f),
        }
    }
}

impl From<SqlxError> for DeleteNodeError {
    fn from(value: SqlxError) -> Self {
        Self::Sqlx(value)
    }
}

impl From<PoolError<SqlxError>> for DeleteNodeError {
    fn from(value: PoolError<SqlxError>) -> Self {
        Self::Pool(value)
    }
}
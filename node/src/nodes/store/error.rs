use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CreateNodeError {
    Diesel(diesel::result::Error),
    Pool(r2d2::Error),
    Parse(url::ParseError),
}

impl Error for CreateNodeError { }

impl Display for CreateNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Diesel(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f),
            Self::Parse(p) => p.fmt(f)
        }
    }
}

impl From<diesel::result::Error> for CreateNodeError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<r2d2::Error> for CreateNodeError {
    fn from(value: r2d2::Error) -> Self {
        Self::Pool(value)
    }
}

impl From<url::ParseError> for CreateNodeError {
    fn from(value: url::ParseError) -> Self {
        Self::Parse(value)
    }
}

#[derive(Debug)]
pub enum GetNodeError {
    Diesel(diesel::result::Error),
    Pool(r2d2::Error),
    Parse(url::ParseError),
}

impl Error for GetNodeError { }

impl Display for GetNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Diesel(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f),
            Self::Parse(p) => p.fmt(f)
        }
    }
}

impl From<diesel::result::Error> for GetNodeError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<r2d2::Error> for GetNodeError {
    fn from(value: r2d2::Error) -> Self {
        Self::Pool(value)
    }
}

impl From<url::ParseError> for GetNodeError {
    fn from(value: url::ParseError) -> Self {
        Self::Parse(value)
    }
}

#[derive(Debug)]
pub enum DeleteNodeError {
    Diesel(diesel::result::Error),
    Pool(r2d2::Error),
}

impl Error for DeleteNodeError { }

impl Display for DeleteNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Diesel(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f),
        }
    }
}

impl From<diesel::result::Error> for DeleteNodeError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<r2d2::Error> for DeleteNodeError {
    fn from(value: r2d2::Error) -> Self {
        Self::Pool(value)
    }
}
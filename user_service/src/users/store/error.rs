use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CreateUserError {
    Diesel(diesel::result::Error),
    Pool(r2d2::Error),
}

impl Error for CreateUserError { }

impl Display for CreateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Diesel(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f)
        }
    }
}

impl From<diesel::result::Error> for CreateUserError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<r2d2::Error> for CreateUserError {
    fn from(value: r2d2::Error) -> Self {
        Self::Pool(value)
    }
}

#[derive(Debug)]
pub enum GetUserError {
    Diesel(diesel::result::Error),
    Pool(r2d2::Error),
}

impl Error for GetUserError { }

impl Display for GetUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Diesel(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f)
        }
    }
}

impl From<diesel::result::Error> for GetUserError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<r2d2::Error> for GetUserError {
    fn from(value: r2d2::Error) -> Self {
        Self::Pool(value)
    }
}

#[derive(Debug)]
pub enum DeleteUserError {
    Diesel(diesel::result::Error),
    Pool(r2d2::Error),
}

impl Error for DeleteUserError { }

impl Display for DeleteUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Diesel(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f)
        }
    }
}

impl From<diesel::result::Error> for DeleteUserError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<r2d2::Error> for DeleteUserError {
    fn from(value: r2d2::Error) -> Self {
        Self::Pool(value)
    }
}

#[derive(Debug)]
pub enum GetPasswordError {
    Diesel(diesel::result::Error),
    Pool(r2d2::Error),
}

impl Error for GetPasswordError { }

impl Display for GetPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Diesel(d) => d.fmt(f),
            Self::Pool(r) => r.fmt(f)
        }
    }
}

impl From<diesel::result::Error> for GetPasswordError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<r2d2::Error> for GetPasswordError {
    fn from(value: r2d2::Error) -> Self {
        Self::Pool(value)
    }
}
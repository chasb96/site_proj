use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum CreateFileError {
    Diesel(diesel::result::Error),
    Pool(r2d2::Error),
    FileSystem(io::Error),
}

impl Error for CreateFileError { }

impl Display for CreateFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateFileError::Diesel(e) => e.fmt(f),
            CreateFileError::Pool(e) => e.fmt(f),
            CreateFileError::FileSystem(e) => e.fmt(f)
        }
    }
}

impl From<diesel::result::Error> for CreateFileError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<r2d2::Error> for CreateFileError {
    fn from(value: r2d2::Error) -> Self {
        Self::Pool(value)
    }
}

impl From<io::Error> for CreateFileError {
    fn from(value: io::Error) -> Self {
        Self::FileSystem(value)
    }
}
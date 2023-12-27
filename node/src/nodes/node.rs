use std::{fmt::Display, error::Error, num::TryFromIntError};
use sqlx::{postgres::PgRow, Row};
use url::{Host, ParseError};

pub struct Node {
    pub id: i32,
    pub name: String,
    pub host: Host,
    pub port: u16,
}

#[derive(Debug)]
pub enum ParsePgNodeError {
    Host(ParseError),
    Port(TryFromIntError),
}

impl Error for ParsePgNodeError { }

impl Display for ParsePgNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePgNodeError::Host(e) => e.fmt(f),
            ParsePgNodeError::Port(e) => e.fmt(f),
        }
    }
}

impl From<ParseError> for ParsePgNodeError {
    fn from(value: ParseError) -> Self {
        Self::Host(value)
    }
}

impl From<TryFromIntError> for ParsePgNodeError {
    fn from(value: TryFromIntError) -> Self {
        Self::Port(value)
    }
}

impl TryFrom<PgRow> for Node {
    type Error = ParsePgNodeError;

    fn try_from(row: PgRow) -> Result<Self, Self::Error> {
        let host_str: &str = row.get("host");
        let port: i32 = row.get("port");

        Ok(Node {
            id: row.get("id"),
            name: row.get("name"),
            host: Host::parse(host_str)?,
            port: u16::try_from(port)?,
        })
    }
}
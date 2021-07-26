use super::function::Function;
use std::convert::TryFrom;
use std::error::Error;
use std::str;
use std::fmt::{Debug, Display, Formatter, Result as FmtRes};

pub struct Request {
    function: Function,
    path: String,
    query: Option<String>,
}

impl Request {
    fn convert_to_request(buffer: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = RequestError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer).or(Err(RequestError::InvalidEncoding))?;
        unimplemented!()
    }
}

pub enum RequestError {
    InvalidEncoding,
    InvalidProtocol,
    InvalidFunction,
    InvalidRequest,
}

impl RequestError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidFunction => "InvalidFunction",
            Self::InvalidRequest => "InvalidRequest",
        }
    }
}

impl Error for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        write!(f, "{}", self.message())
    }
}

impl Debug for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        write!(f, "{}", self.message())
    }
}

use super::function::Function;
use super::query_map::Query;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtRes};
use std::str;

#[derive(Debug)]
pub struct Request<'buf_life> {
    function: Function,
    path: &'buf_life str,
    query: Option<Query<'buf_life>>,
}

// impl <'buf_life> Request<'buf_life> {
//     fn convert_to_request(buffer: &'buf_life [u8]) -> Result<Self, String> {
//         unimplemented!();
//     }
// }

impl<'buf_life> TryFrom<&'buf_life [u8]> for Request<'buf_life> {
    type Error = RequestError;

    fn try_from(buffer: &'buf_life [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer).or(Err(RequestError::InvalidEncoding))?;

        let (function, rest_of_request) =
            explode(request, ' ').ok_or(RequestError::InvalidRequest)?;
        let (path_and_query, rest_of_request) =
            explode(rest_of_request, ' ').ok_or(RequestError::InvalidRequest)?;
        let (protocol, _) = explode(rest_of_request, ' ').ok_or(RequestError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(RequestError::InvalidProtocol);
        }

        let function: Function = function.parse()?;
        let mut path = path_and_query;
        let mut query = None;

        if let Some((found_path, found_query)) = explode(path_and_query, '?') {
            path = found_path;
            query = Some(Query::from(found_query));
        }

        Ok(Self {
            function,
            path,
            query,
        })
    }
}

fn explode(text: &str, sep: char) -> Option<(&str, &str)> {
    for (index, letter) in text.chars().enumerate() {
        if letter == sep || letter == '\r' {
            return Some((&text[..index], &text[index + 1..]));
        }
    }
    return None;
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

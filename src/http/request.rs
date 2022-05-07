use std::error::Error;
use std::fmt::{Debug, Display};
use std::str::Utf8Error;
use crate::{Method, MethodError};

// use super::method::Method;
pub struct Request {
    method: Method,
    path: String,
    query: Option<String>, //absent if not present
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}
impl From<MethodError> for ParseError{
    fn from(_: Method) -> Self {
        ParseError::InvalidMethod
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid request",
            ParseError::InvalidEncoding => "Invalid encoding",
            ParseError::InvalidProtocol => "Invalid protocol",
            ParseError::InvalidMethod => "Invalid method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        self.message()
    }
}

//types conversion
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let request = std::str::from_utf8(buffer)?; //bytes slice to string


        let (method, request)=get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request)=get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _)=get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> { //return word and rest of the string
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' || c == '\n' {
            return Some((&request[..i], &request[i + 1..])); //safe call, the next char is guaranteed to exist, is a blank space -> 1 byte
        }
    }
    None
}


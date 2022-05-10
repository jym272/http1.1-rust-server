use std::error::Error;
use std::fmt::{Debug, Display};
use std::str::Utf8Error;
use crate::http::{Method, MethodError, Query};

#[derive(Debug)]
pub struct Request<'buffer> {
    method: Method,
    path: &'buffer str,
    //only want to read it, is ref to the buffer. Dangling reference if the buffer is dropped. Use after freeing the buffer.
    query: Option<Query<'buffer>>, //absent if not present
}

impl Request<'_> {
    //getters
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }

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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
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
impl<'buffer> TryFrom<&'buffer [u8]> for Request<'buffer> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n

    fn try_from(buffer: &'buffer [u8]) -> Result<Self, Self::Error> {
        let request = std::str::from_utf8(buffer)?; //bytes slice to string


        //method, path, protocal has the same lifetime as the buffer
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query = None;

        if let Some(index) = path.find('?') {
            query = Some(Query::from(&path[index + 1..]));
            path = &path[..index];
        }

        Ok(Request{
            method,
            path,
            query
        })


        // unimplemented!()
    }
}
//Slice in arg is a reference to the buffer.
//the slices returned are references to the buffer too, it has the same lifetime as the buffer.
//don´t need to specify lifetime because the buffer has the same lifetime as the request.
//If there is another argument in the function, it has to be specified.
fn get_next_word(request: &str) -> Option<(&str, &str)> { //return word and rest of the string
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' || c == '\n' {
            return Some((&request[..i], &request[i + 1..])); //safe call, the next char is guaranteed to exist, is a blank space -> 1 byte
        }
    }
    None
}


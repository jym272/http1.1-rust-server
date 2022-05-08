use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    RequestEntityTooLarge = 413,
    RequestURITooLong = 414,
    UnsupportedMediaType = 415,
    RequestedRangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
}

 impl StatusCode {
    // pub fn from_u16(code: u16) -> Option<StatusCode> {
    //     match code {
    //         200 => Some(StatusCode::Ok),
    //         201 => Some(StatusCode::Created),
    //         202 => Some(StatusCode::Accepted),
    //         204 => Some(StatusCode::NoContent),
    //         400 => Some(StatusCode::BadRequest),
    //         401 => Some(StatusCode::Unauthorized),
    //         403 => Some(StatusCode::Forbidden),
    //         404 => Some(StatusCode::NotFound),
    //         405 => Some(StatusCode::MethodNotAllowed),
    //         406 => Some(StatusCode::NotAcceptable),
    //         408 => Some(StatusCode::RequestTimeout),
    //         409 => Some(StatusCode::Conflict),
    //         410 => Some(StatusCode::Gone),
    //         411 => Some(StatusCode::LengthRequired),
    //         412 => Some(StatusCode::PreconditionFailed),
    //         413 => Some(StatusCode::RequestEntityTooLarge),
    //         414 => Some(StatusCode::RequestURITooLong),
    //         415 => Some(StatusCode::UnsupportedMediaType),
    //         416 => Some(StatusCode::RequestedRangeNotSatisfiable),
    //         417 => Some(StatusCode::ExpectationFailed),
    //         500 => Some(StatusCode::InternalServerError),
    //         501 => Some(StatusCode::NotImplemented),
    //         502 => Some(StatusCode::BadGateway),
    //         503 => Some(StatusCode::ServiceUnavailable),
    //         504 => Some(StatusCode::GatewayTimeout),
    //         505 => Some(StatusCode::HttpVersionNotSupported),
    //         _ => None,
    //     }
    // }
    pub fn reason(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NoContent => "No Content",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::NotAcceptable => "Not Acceptable",
            StatusCode::RequestTimeout => "Request Timeout",
            StatusCode::Conflict => "Conflict",
            StatusCode::Gone => "Gone",
            StatusCode::LengthRequired => "Length Required",
            StatusCode::PreconditionFailed => "Precondition Failed",
            StatusCode::RequestEntityTooLarge => "Request Entity Too Large",
            StatusCode::RequestURITooLong => "Request-URI Too Long",
            StatusCode::UnsupportedMediaType => "Unsupported Media Type",
            StatusCode::RequestedRangeNotSatisfiable => "Requested Range Not Satisfiable",
            StatusCode::ExpectationFailed => "Expectation Failed",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
            StatusCode::GatewayTimeout => "Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported",
        }

    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", *self as u16)
    }

}
use super::method::Method;
#[allow(dead_code)]
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

use std::convert::TryFrom;
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        use std::str;
        let request = str::from_utf8(buf)?;

        unimplemented!()
    }
}

#[allow(dead_code)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocal,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocal => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use std::fmt::Debug;
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

use std::fmt::Display;
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

use std::convert::From;
use std::str::Utf8Error;
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

use std::error::Error;
impl Error for ParseError {}

use super::method::Method;

#[derive(Debug)]
pub struct Request<'buf> {
    method:       Method,
    path:         &'buf str,
    query_string: Option<QueryString<'buf>>,
}

impl<'buf> Request<'buf> {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

use super::QueryString;
use std::convert::TryFrom;
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        use std::str;
        let request = str::from_utf8(buf)?;

        let (method, request) =
            get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) =
            get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) =
            get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocal);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1 ..]));
            path = &path[.. i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[.. i], &&request[i + 1 ..]));
        }
    }
    None
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
            | Self::InvalidRequest => "Invalid Request",
            | Self::InvalidEncoding => "Invalid Encoding",
            | Self::InvalidProtocal => "Invalid Protocol",
            | Self::InvalidMethod => "Invalid Method",
        }
    }
}

use std::fmt::{Debug, Formatter, Result as FmtResult};
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

use super::method::MethodError;
use std::convert::From;
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidEncoding
    }
}

use std::str::Utf8Error;
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

use std::error::Error;
impl Error for ParseError {}

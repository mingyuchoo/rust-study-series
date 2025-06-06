use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body:        Option<String>,
}

use std::io::{Result as IoResult, Write};

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body,
        }
    }

    /// dyn: Dynamic Dispatch
    /// impl: Static Dispatch
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            | Some(b) => b,
            | None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body,
        )
    }
}

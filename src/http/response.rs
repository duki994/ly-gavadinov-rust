use super::status_code::StatusCode;
use std::io::{Result as IoResult, Write};
use std::{
    fmt::{Display, Result as FmtResult},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }


    // impl `Trait` does static dispatch - compiler will copy-paste and create
    // separate functions (fn. signatures) for each call of different type implementing trait `Trait` 
    // we use in our codebase when calling this function
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

use std::io::{BufRead, BufReader, Read};

pub struct HttpRequest {
    method: Method,
    path: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

pub type Method = String;

impl HttpRequest {
    pub fn new(
        method: Method,
        path: String,
        headers: Vec<(String, String)>,
        body: Vec<u8>,
    ) -> HttpRequest {
        Self {
            method,
            path,
            headers,
            body,
        }
    }

    pub fn from_stream(stream: &mut impl Read) -> Result<HttpRequest> {
        let mut buffered_reader = BufReader::new(stream);
        let mut start_line: Vec<u8> = vec![0; 512];
        let start_line_length = buffered_reader.read_until('\r\n', start_line)?;


    }
}
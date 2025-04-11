/* HTTP Parser

https://datatracker.ietf.org/doc/html/rfc9112#name-message-format
*/
use std::io::{BufRead, BufReader};

struct ParseError {
    message: String,
}

enum HttpState {
    StartLine,
    Headers,
    Body,
    Done,
}

struct HttpParser {
    state: HttpState,
    buffer: Vec<u8>,
}

struct HttpMessage {
    start_line: String,
    headers: Vec<(String, String)>,
    body: BufReader<u8>,
}

impl HttpParser {
    pub fn new() -> Self {
        Self {
            state: HttpState::StartLine,
            buffer: Vec::new(),
        }
    }

    pub fn parse(&mut self, reader: &mut impl BufRead) -> Result<HttpMessage, ParseError> {
        Err(ParseError {
            message: "".to_string(),
        })
    }
}

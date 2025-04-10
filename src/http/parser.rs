/*  */

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

impl HttpParser {}

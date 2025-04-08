use std::io::Read;

pub struct HttpRequest {
    method: Method,
    path: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

pub type Method = String;

impl HttpRequest {
    pub fn new(method: Method, path: String, headers: Vec<(String, String)>, body: Vec<u8>) -> HttpRequest {
        HttpRequest {
            method,
            path,
            headers,
            body
        }
    }

    pub fn from_stream(stream: &dyn Read) -> HttpRequest {
        let mut buffer: [u8; 512] = [0; 512];
        stream.read(&mut buffer);
        HttpRequest::new(
            
        )
    }
}
mod error;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use error::PathError;

const GET: &str = "GET";

fn main() {
    println!("Starting server...");
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap_or_else(|err| panic!("first panic:{}", err));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut request_line = request_line.split_whitespace();

    let _request_method = request_line.next().unwrap_or(GET);
    let request_path = request_line.next().unwrap_or("/");
    let _request_version = request_line.next().unwrap();

    let file_path = match request_path {
        "/color" => handle_get("data/color.json"),
        _ => Err(PathError::new("Bad request path")),
    };
    match file_path {
        Ok(content) => {
            stream
                .write_all(create_response(&content).as_bytes())
                .unwrap();
        }
        Err(_) => {
            stream.write_all(send_error().as_bytes()).unwrap();
        }
    };
}

fn create_response(content: &str) -> String {
    let length = content.len();
    format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\n\r\n{content}")
}

fn send_error() -> String {
    format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\nAccess-Control-Allow-Origin: *\r\n\r\n")
}

fn handle_get(path: &str) -> Result<String, PathError> {
    let mut content = String::new();
    File::open(path).unwrap().read_to_string(&mut content).unwrap();
    Ok(content)
}

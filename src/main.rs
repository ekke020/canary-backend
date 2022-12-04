mod error;

use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use error::PathError;

const GET: &str = "GET";

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PATH_RESOURCES: HashMap<&'static str, &'static str> =
        HashMap::from([("/color", "data/color.json")]);
}

fn main() {
    println!("Starting server...");
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap_or_else(|err| panic!("{}", err));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", stream);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    println!("{:?}", buf_reader.lines());
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut request_line = request_line.split_whitespace();

    let request_method = request_line.next().unwrap_or(GET);
    let request_path = request_line.next().unwrap_or("/");
    let _request_version = request_line.next().unwrap();

    let file_path = match request_method {
        GET => handle_get(request_path),
        _ => panic!("This should not happen"),
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
    let path = PATH_RESOURCES.get(path).ok_or(PathError::new(""))?;
    File::open(path).unwrap().read_to_string(&mut content).unwrap();
    Ok(content)
}

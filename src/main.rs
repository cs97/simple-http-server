
use std::{
    fs,
    path::Path,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod status_codes;
use crate::status_codes::*;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    //let listener = TcpListener::bind("192.168.178.53:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    handle_request(&http_request, stream);

}


fn handle_request(http_request: &Vec<String>, mut stream: TcpStream) {

    // Example authentication. Please use a secure authentication method
    //let pass = "dXNlcjpwYXNz";
    let pass = "none";
    if &extract_var("Authorization", 2, &http_request) == pass {

        let request: &str = &extract_var("HTTP", 0, &http_request);
        let path: &str = &extract_var("HTTP", 1, &http_request)[1..];

        match request {
        //match extract_var("HTTP", 0, &http_request) {
            "GET" => handle_get(&path, stream),
            "PUT" => handle_put(&path, &extract_var("Content-Length", 1, &http_request), stream),
            _ => stream.write_all(not_found_404().as_bytes()).unwrap(),
        }

    } else {
        stream.write_all(unauthorized_401().as_bytes()).unwrap();
    }
}


fn extract_var(s: &str, position: usize, http_request: &Vec<String>) -> String {

    for n in http_request{

        if n.contains(&s) {
            let content_length_vec: Vec<_> = n.split(' ').collect();
            return content_length_vec[position].to_string();
        }

    }
    return "none".to_string();
}


fn handle_get(mut path: &str, mut stream: TcpStream) {

    if path.len() < 1{
        path = "index.html";
    }

    if Path::new(path).is_file() {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read(path).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();

    } else {
        stream.write_all(not_found_404().as_bytes()).unwrap();
    }
}


fn handle_put(path: &str, content_length: &str, mut stream: TcpStream) {

    let length = content_length.parse::<usize>().unwrap();

    if path.len() < 1 {
        stream.write_all("HTTP/1.1 400 Bad Request\r\n".as_bytes()).unwrap();
        return
    }

    let mut buffer = vec![0; length];
    stream.read_exact(&mut buffer).unwrap();
    fs::write(path, buffer).unwrap();

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(status_line.as_bytes()).unwrap();
}


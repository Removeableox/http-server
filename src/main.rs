use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Result, Write};
use std::fs;

fn good_connection_res() -> String {
    let status = "HTTP/1.1 200 OK";
    let file = fs::read_to_string("src/html/index.html").unwrap();
    let file_length = file.len();

    format!("{status}\r\nContent-Length: {file_length}\r\n\r\n{file}")
}

fn bad_connection_res() -> String {
    let status = "HTTP/1.1 404 NOT FOUND";
    let file = fs::read_to_string("src/html/404.html").unwrap();
    let file_length = file.len();

    format!("{status}\r\nContent-Length: {file_length}\r\n\r\n{file}")
}

fn handle_connection(mut stream: TcpStream) {
    // get request info
    let reader = BufReader::new(&stream); 
    let request = reader.lines().next().unwrap().unwrap();

    let response = match request.as_str() {
        "GET / HTTP/1.1" => good_connection_res(),
        _ => bad_connection_res(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }

    Ok(())
}

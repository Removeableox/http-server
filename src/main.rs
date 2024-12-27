use std::net::TcpListener;
use std::io::{BufRead, BufReader, Result, Write};
use std::fs;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        let mut tcp_stream = stream.unwrap();
        let reader = BufReader::new(&tcp_stream); 

        let http_request: Vec<_> = reader
            .lines()
            .map(|line| line.unwrap()) // unwrap the line variables
            .take_while(|line| !line.is_empty()) // get rid of empty lines
            .collect();

        let status = "HTTP/1.1 200 OK";
        let file = fs::read_to_string("src/html/index.html").unwrap();
        let file_length = file.len();

        let res = format!("{status}\r\nContent-Length: {file_length}\r\n\r\n{file}");

        tcp_stream.write_all(res.as_bytes()).unwrap();
    }

    Ok(())
}

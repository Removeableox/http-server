mod threads;
use threads::ThreadPool;
use std::{
    net::{TcpListener, TcpStream},
    io::{BufReader, Write, BufRead},
    fs,
};

fn handle_connection(mut stream: TcpStream) {
    // get request info
    let reader = BufReader::new(&stream); 
    let request = reader.lines().next().unwrap().unwrap();

    let (status, file_name) = match request.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let file = fs::read_to_string(format!("src/html/{file_name}")).unwrap();
    let size = file.len();

    let response = format!("{status}\r\nContent-Length: {size}\r\n\r\n{file}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        pool.execute(|| handle_connection(stream.unwrap()));
    }

    println!("Shutting down.");
}

use std::io::{prelude::*, Read};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_code, file_name) = if buffer.starts_with(get) {
        ("200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "index.html")
    } else {
        ("404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();

        thread_pool.execute(|| handle_connection(stream));
    }
}

use std::io::prelude::*;

use std::net::TcpListener;
use std::net::TcpStream;

use std::fs;
use std::path::Path;

use crate::threadpool;

pub fn start_admin_page(address: String) {
    let listener = TcpListener::bind(&address).unwrap();
    let pool = threadpool::ThreadPool::new(5);
    println!("Web server started, listening on {}", address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("request:{}", String::from_utf8_lossy(&buffer));

    let get = b"GET /admin HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./gui/admin.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./gui/404.html")
    };

    let contents = fs::read_to_string(Path::new(filename)).unwrap();

    println!("html:{}", contents);

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

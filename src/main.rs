use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod portscanner;
mod threadpool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    let pool = threadpool::ThreadPool::new(5);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut stream: TcpStream) {
    // read request from client
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    // proxy servers fowards the request to desired URI

    let status_line = if buffer.starts_with(get) {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 NOT FOUND"
    };
    println!("Request:{}", String::from_utf8_lossy(&buffer));
    let response = format!("{}", status_line);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_forward() {
    // find remote ip address of the host by using the ToSocketsAddrs

    // create TCP stream connection to host TCPStream::connect

    // send the request to the host stream.write(), stream.flush() out request

    //wait for the response and read it stream.read()
    // https://www.jmarshall.com/easy/http/

    //return response and close connection
}

use std::io::prelude::*;
use std::net::Shutdown;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
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
    let mut clientbuffer = [0; 1024];
    let mut proxybuffer = [0; 1024];
    stream.read(&mut clientbuffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    // proxy servers fowards the request to desired URI
    let req = String::from_utf8_lossy(&clientbuffer).into_owned();
    println!("Request:{}", req);
    handle_forward(&req, &mut proxybuffer);
    println!(
        "Response from server:{}",
        String::from_utf8_lossy(&proxybuffer)
    );
    stream.write(&proxybuffer).unwrap();
    stream.flush().unwrap();
}

fn handle_forward(req: &String, buffer: &mut [u8]) {
    // find remote ip address of the host by using the ToSocketsAddrs, http is on port 80
    // make sure to have extra line at the very end of the http req otherwise doesn't work
    let request =
        "GET /~carey/CPSC441/ass1/test1.html HTTP/1.1\r\nHost: pages.cpsc.ucalgary.ca\r\n\r\n";
    println!("client's request: {}", req);
    // create TCP stream connection to host TCPStream::connect
    let mut stream = TcpStream::connect("pages.cpsc.ucalgary.ca:80").unwrap();
    // send the request to the host stream.write(), stream.flush() out request
    stream.write(request.as_bytes()).unwrap();
    stream.flush().unwrap();
    //wait for the response and read it stream.read()
    // https://www.jmarshall.com/easy/http/

    stream.read(buffer).unwrap();
    //return response and close connection
    stream.shutdown(Shutdown::Both).unwrap();
}

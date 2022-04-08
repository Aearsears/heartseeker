use std::io::prelude::*;
use std::io::BufReader;

use std::net::Shutdown;
use std::net::TcpListener;
use std::net::TcpStream;

use std::time::Instant;

use crate::threadpool;

pub fn start_proxy(address: String) {
    // TODO: dev move and prod mode?
    // TODO: tests
    let listener = TcpListener::bind(&address).unwrap();
    let pool = threadpool::ThreadPool::new(5);
    println!("Proxy server started, listening on {}", address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read request from client
    let mut clientbuffer = String::with_capacity(1024);
    let mut proxybuffer = [0; 1024];
    let mut reader = BufReader::with_capacity(1024, &stream);
    // cannot read one line, need to read line until hit only two CRLF character and then break loop
    let crlf = String::from("\r\n\r\n");
    let mut checkcrlf = String::with_capacity(1024);
    while !checkcrlf.ends_with(&crlf) {
        reader.read_line(&mut checkcrlf).unwrap();
        println!("client input: {}", checkcrlf);
    }

    clientbuffer.push_str(&checkcrlf);
    // proxy servers fowards the request to desired URI
    let req = clientbuffer.clone();
    println!("Request:{:?}", req);
    let now = Instant::now();
    handle_forward(&req, &mut proxybuffer);
    println!("Response duration: {:?}", now.elapsed());
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
    // http://pages.cpsc.ucalgary.ca/~carey/CPSC441/ass1/test1.html
    // "GET /~carey/CPSC441/ass1/test1.html HTTP/1.1\r\nHost: pages.cpsc.ucalgary.ca\r\n\r\n";

    // create TCP stream connection to host
    let hostname: &str = get_hostname(req);
    println!("hostname:{}", hostname);

    let mut stream = TcpStream::connect(format!("{}{}", &hostname, &String::from(":80"))).unwrap();
    // send the request to the host stream.write(), stream.flush() out request
    stream.write(req.as_bytes()).unwrap();
    stream.flush().unwrap();
    //wait for the response and read it stream.read()
    // https://www.jmarshall.com/easy/http/

    stream.read(buffer).unwrap();
    //return response and close connection
    stream.shutdown(Shutdown::Both).unwrap();
}

fn get_hostname<'a>(request: &'a String) -> &'a str {
    let split: Vec<&str> = request.split("\r\n").collect();
    let mut hostsplit: Vec<&str> = Vec::new();
    for elem in &split {
        if elem.starts_with("Host: ") {
            hostsplit = elem.split("Host: ").collect();
        }
    }
    match hostsplit.get(1) {
        Some(&str) => &str,
        None => {
            panic!("No hostname in request!")
        }
    }
}

use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

use std::net::Shutdown;
use std::net::TcpListener;
use std::net::TcpStream;

use std::time::Instant;

use crate::threadpool;

const HEADERSIZE: usize = 2000;

pub fn start_proxy(address: String) {
    // TODO: dev move and prod mode?
    // TODO: tests
    // TODO: abstract functions between proxy and webserver
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

fn handle_connection(stream: TcpStream) {
    // read request from client
    let mut clientreq = String::with_capacity(HEADERSIZE);
    let mut proxyreq = String::with_capacity(HEADERSIZE);

    let mut reader = BufReader::with_capacity(HEADERSIZE, &stream);
    let mut writer = BufWriter::new(&stream);
    // cannot read one line, need to read line until hit only two CRLF character and then break loop
    let crlf = String::from("\r\n\r\n");
    while !clientreq.ends_with(&crlf) {
        reader.read_line(&mut clientreq).unwrap();
    }

    // proxy servers fowards the request to desired URI
    println!("Request:{:?}", clientreq);
    let now = Instant::now();
    let proxyres = handle_forward(&clientreq, &mut proxyreq);
    println!("Response duration: {:?}", now.elapsed());
    println!("Response from server:{:?}", &proxyreq);
    println!("Response body from server:{}", &proxyres);
    writer.write(proxyreq.as_bytes()).unwrap();
    writer.write(proxyres.as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn handle_forward(req: &String, buffer: &mut String) -> String {
    // find remote ip address of the host by using the ToSocketsAddrs, http is on port 80
    // make sure to have extra line at the very end of the http req otherwise doesn't work
    // http://pages.cpsc.ucalgary.ca/~carey/CPSC441/ass1/test1.html
    // "GET /~carey/CPSC441/ass1/test1.html HTTP/1.1\r\nHost: pages.cpsc.ucalgary.ca\r\n\r\n";

    // create TCP stream connection to host
    let hostname: &str = get_hostname(req);
    println!("hostname:{}", hostname);

    let stream = TcpStream::connect(format!("{}{}", &hostname, &String::from(":80"))).unwrap();
    let mut reader = BufReader::with_capacity(HEADERSIZE, &stream);
    let mut writer = BufWriter::new(&stream);
    // send the request to the host stream.write(), stream.flush() out request
    writer.write(req.as_bytes()).unwrap();
    writer.flush().unwrap();
    //wait for the response and read it stream.read()
    /*
    https://www.jmarshall.com/easy/http/
    For HTTP protocol 1.0 the connection closing was used to signal the end of data.

    This was improved in HTTP 1.1 which supports persistant connections. For HTTP 1.1 typically you set or read the Content-Length header to know how much data to expect.

    Finally with HTTP 1.1 there is also the possibility of "Chunked" mode, you get the size as they come and you know you've reached the end when a chunk Size == 0 is found.
    */
    let crlf = String::from("\r\n\r\n");
    while !buffer.ends_with(&crlf) {
        reader.read_line(buffer).unwrap();
    }
    //get the content length
    let mut body: [u8; 245] = [0; 245];
    //return response and close connection
    reader.read_exact(&mut body).unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
    String::from_utf8_lossy(&body).into_owned()
}
// TODO: parse response headers into a hashmap
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

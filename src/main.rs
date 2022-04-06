use std::io::prelude::*;
use std::io::BufReader;
use std::net::Shutdown;
use std::net::TcpListener;
use std::net::TcpStream;
mod portscanner;
mod threadpool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    let pool = threadpool::ThreadPool::new(5);
    println!("Proxy server started, listening on port 4000...");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut stream: TcpStream) {
    loop {
        // read request from client
        // need to read until newline so that can read entire request for testing with telnet"
        let mut clientbuffer = String::with_capacity(1024);
        let mut proxybuffer = [0; 1024];
        let mut reader = BufReader::with_capacity(1024, &stream);
        // cannot read one line, need to read line until hit only only CRLF character and then break loop
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
        handle_forward(&req, &mut proxybuffer);
        println!(
            "Response from server:{}",
            String::from_utf8_lossy(&proxybuffer)
        );
        stream.write(&proxybuffer).unwrap();
        stream.flush().unwrap();
    }
}

fn handle_forward(req: &String, buffer: &mut [u8]) {
    // find remote ip address of the host by using the ToSocketsAddrs, http is on port 80
    // make sure to have extra line at the very end of the http req otherwise doesn't work
    // http://pages.cpsc.ucalgary.ca/~carey/CPSC441/ass1/test1.html
    // let request =
    // "GET /~carey/CPSC441/ass1/test1.html HTTP/1.1\r\nHost: pages.cpsc.ucalgary.ca\r\n\r\n";
    let split: Vec<&str> = req.split("\r\n").collect();
    let mut hostsplit: Vec<&str> = Vec::new();
    for elem in &split {
        if elem.starts_with("Host: ") {
            hostsplit = elem.split("Host: ").collect();
        }
    }
    // TODO: parse host name from request
    // create TCP stream connection to host TCPStream::connect
    let hostname = hostsplit.get(1);
    println!("hostname:{}", hostname.unwrap());
    match hostname {
        Some(str) => {
            let mut stream =
                TcpStream::connect(format!("{}{}", *hostname.unwrap(), &String::from(":80")))
                    .unwrap();
            // send the request to the host stream.write(), stream.flush() out request
            stream.write(req.as_bytes()).unwrap();
            stream.flush().unwrap();
            //wait for the response and read it stream.read()
            // https://www.jmarshall.com/easy/http/

            stream.read(buffer).unwrap();
            //return response and close connection
            stream.shutdown(Shutdown::Both).unwrap();
        }
        None => {
            panic!("No hostname in request!")
        }
    }
}

// fn get_hostname<'a>(request: &'a str) -> &&'a str {
//     let request = "GET /~carey/CPSC441/ass1/test1.html HTTP/1.1\r\nHost: pages.cpsc.ucalgary.ca\r\nConnection: keep-alive\r\n\r\n";
//     let split: Vec<&str> = request.split("\r\n").collect();
//     // let hostsplit: Vec<&str>;
//     for elem in &split {
//         if elem.starts_with("Host: ") {
//             let hostsplit: Vec<&str> = elem.split("Host: ").collect();
//             hostsplit.get(1).unwrap()
//         }
//     }
// }

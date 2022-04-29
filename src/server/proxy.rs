use serde::{Deserialize, Serialize};
use serde_json::Result;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};

use std::time::Duration;
use std::time::Instant;

use crate::io;
use crate::utility;
use crate::utility::Transactions;

const HEADERSIZE: usize = 2000;

#[derive(Serialize, Deserialize)]
struct Flow {
    duration: u128,
    request: String,
    response: String,
}

#[tokio::main]
pub async fn start_proxy(address: String) {
    // TODO: dev move and prod mode?
    // TODO: tests
    let listener = TcpListener::bind(&address).await.unwrap();
    println!("Proxy server started, listening on {}", address);
    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) {
    // read request from client
    let mut clientreq = String::with_capacity(HEADERSIZE);
    let mut proxyreq = String::with_capacity(HEADERSIZE);

    let mut reader = BufReader::with_capacity(HEADERSIZE, &mut stream);
    // cannot read one line, need to read line until hit only two CRLF character and then break loop
    let crlf = String::from("\r\n\r\n");
    while !clientreq.ends_with(&crlf) {
        reader.read_line(&mut clientreq).await;
    }

    // proxy servers fowards the request to desired URI
    println!("Request:{:?}", clientreq);
    let now = Instant::now();
    let proxyres = handle_forward(&clientreq, &mut proxyreq).await;
    let duration = now.elapsed();
    println!("Response duration: {:?}", duration);
    println!("Response from server:{:?}", &proxyreq);
    println!("Response body from server:{}", &proxyres);

    let mut writer = BufWriter::new(&mut stream);
    writer.write(proxyreq.as_bytes()).await;
    writer.write(proxyres.as_bytes()).await;
    writer.flush().await;

    let flow = Flow {
        duration: duration.as_millis(),
        request: clientreq,
        response: format!("{}{}", proxyreq, proxyres),
    };
    let j = serde_json::to_string(&flow).unwrap();
    io::inflow_outflow_to_file(j);
}

async fn handle_forward(req: &String, buffer: &mut String) -> String {
    // find remote ip address of the host by using the ToSocketsAddrs, http is on port 80
    // make sure to have extra line at the very end of the http req otherwise doesn't work
    // http://pages.cpsc.ucalgary.ca/~carey/CPSC441/ass1/test1.html
    // "GET /~carey/CPSC441/ass1/test1.html HTTP/1.1\r\nHost: pages.cpsc.ucalgary.ca\r\n\r\n";

    // create TCP stream connection to host
    let hostname: &str = match get_hostname(req) {
        Some(s) => s,
        None => return "No hostname".to_string(),
    };
    println!("hostname:{}", hostname);

    let mut stream = TcpStream::connect(format!("{}{}", &hostname, &String::from(":80")))
        .await
        .unwrap();
    let mut writer = BufWriter::new(&mut stream);
    // send the request to the host stream.write(), stream.flush() out request
    writer.write(req.as_bytes()).await;
    writer.flush().await;
    //wait for the response and read it stream.read()
    /*
    https://www.jmarshall.com/easy/http/
    For HTTP protocol 1.0 the connection closing was used to signal the end of data.

    This was improved in HTTP 1.1 which supports persistant connections. For HTTP 1.1 typically you set or read the Content-Length header to know how much data to expect.

    Finally with HTTP 1.1 there is also the possibility of "Chunked" mode, you get the size as they come and you know you've reached the end when a chunk Size == 0 is found.
    */
    let mut reader = BufReader::with_capacity(HEADERSIZE, &mut stream);
    let crlf = String::from("\r\n\r\n");
    while !buffer.ends_with(&crlf) {
        reader.read_line(buffer).await;
    }
    //get the content length
    let headers = utility::parse_message(&buffer, Transactions::Res);
    let conlen = headers
        .get("Content-Length")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut body = Vec::with_capacity(conlen);
    //return response and close connection
    reader.read_to_end(&mut body).await;
    stream.shutdown().await;
    String::from_utf8_lossy(&body).into_owned()
}

fn get_hostname<'a>(request: &'a String) -> Option<&'a str> {
    let split: Vec<&str> = request.split("\r\n").collect();
    let mut hostsplit: Vec<&str> = Vec::new();
    for elem in &split {
        if elem.starts_with("Host: ") {
            hostsplit = elem.split("Host: ").collect();
        }
    }
    match hostsplit.get(1) {
        Some(&str) => Some(&str),
        None => None,
    }
}

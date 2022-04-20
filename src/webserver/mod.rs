use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

use std::net::TcpListener;
use std::net::TcpStream;

use std::fs;
use std::path::Path;

use crate::threadpool;

const HEADERSIZE: usize = 2000;
// TODO: handle more verbs, paths
// TODO: write a websockets server
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

fn handle_connection(stream: TcpStream) {
    // read request from client
    let mut clientreq = String::with_capacity(HEADERSIZE);

    let mut reader = BufReader::with_capacity(HEADERSIZE, &stream);
    let mut writer = BufWriter::new(&stream);
    let crlf = String::from("\r\n\r\n");
    while !clientreq.ends_with(&crlf) {
        reader.read_line(&mut clientreq).unwrap();
    }

    // proxy servers fowards the request to desired URI
    println!("Request:{:?}", clientreq);

    let get = "GET /admin HTTP/1.1\r\n";

    let (status_line, filename) = if clientreq.starts_with(get) {
        (
            "HTTP/1.1 200 OK",
            "./src/gui/heartseeker-ui/.next/server/pages/index.html",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND",
            "./src/gui/heartseeker-ui/.next/server/pages/404.html",
        )
    };

    let contents = fs::read_to_string(Path::new(filename)).unwrap();

    println!("html:{}", contents);

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    writer.write(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

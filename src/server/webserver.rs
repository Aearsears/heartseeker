use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

use std::net::TcpListener;
use std::net::TcpStream;

use std::fs;
use std::path::Path;

use crate::threadpool;
use crate::utility;
use crate::utility::Transactions;

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
    let headers = utility::parse_message(&clientreq, Transactions::Req);
    println!("Request:{:?}", clientreq);

    let admin = "/admin";
    let get = "GET";
    let base_path = "./src/gui/heartseeker-ui";
    let full_path = format!(
        "{}{}",
        base_path,
        headers.get("URI").unwrap().replacen("_", ".", 1)
    );
    println!("{}", full_path);
    let (status_line, filename) =
        if headers.get("URI").unwrap() == admin && headers.get("Verb").unwrap() == get {
            (
                "HTTP/1.1 200 OK",
                "./src/gui/heartseeker-ui/.next/server/pages/index.html",
            )
        } else {
            ("HTTP/1.1 200 OK", full_path.as_str())
        };
    //need to handle 404

    let contents = fs::read_to_string(Path::new(filename)).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    writer.write(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

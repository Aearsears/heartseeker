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

    let index = "/";
    let index_path = "./src/gui/heartseeker-ui/.next/server/pages/index.html";
    let err_path = "./src/gui/heartseeker-ui/.next/server/pages/404.html";
    let fallback_err_path = "<!-- HTML5 -->
<!DOCTYPE html>
<html>
  <head>
    <title>404 - Page not found</title>
    <base href=\"\">
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
  </head>
  <body>
    <main>
      <center>
        <br /><br /><br /><br /><br /><br />
  			<h1>404 - Page not found!</h1>
        <h3><a href=\"
        / \">Click here to go back home</a></h3>
        <br /><br /><br /><br />
      </center>
    </main>
  </body>
</html>";
    let get = "GET";
    let base_path = "./src/gui/heartseeker-ui";
    let full_path = format!(
        "{}{}",
        base_path,
        headers.get("URI").unwrap().replacen("_", ".", 1)
    );

    let filename: &str =
        if headers.get("URI").unwrap() == index && headers.get("Verb").unwrap() == get {
            index_path
        } else {
            full_path.as_str()
        };

    let (contents, status_line) = match fs::read_to_string(Path::new(&filename)) {
        Ok(string) => (string, "HTTP/1.1 200 OK"),
        Err(e) => (
            fs::read_to_string(Path::new(&err_path)).unwrap_or(fallback_err_path.to_string()),
            "HTTP/1.1 404 Not Found",
        ),
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    writer.write(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

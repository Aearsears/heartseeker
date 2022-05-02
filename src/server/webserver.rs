use std::collections::HashMap;
use std::fs;
use std::path::Path;

use tokio::io::{AsyncBufReadExt, AsyncWrite, AsyncWriteExt};
use tokio::io::{BufReader, BufWriter};
use tokio::net::TcpListener;

use bytes::BufMut;
use bytes::BytesMut;

use sha1::{Digest, Sha1};

extern crate base64;
use base64::encode;

use crate::utility;
use crate::utility::Transactions;

const HEADERSIZE: usize = 2000;
const FRAMESIZE: usize = 64;
// TODO: handle more verbs, paths
// TODO: write a websockets server

#[tokio::main]
pub async fn start_admin_page(address: String) {
    let listener = TcpListener::bind(&address).await.unwrap();
    println!("Web server started, listening on {}", address);
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

    let mut reader = BufReader::with_capacity(HEADERSIZE, &mut stream);

    let crlf = String::from("\r\n\r\n");
    while !clientreq.ends_with(&crlf) {
        match reader.read_line(&mut clientreq).await {
            Err(e) => {
                eprintln!("Could not read the request from the client. Error: {}", e);
                return;
            }
            _ => {}
        };
    }
    let headers = utility::parse_message(&clientreq, Transactions::Req);
    println!("Request:{:?}", clientreq);

    if (!headers.get("Upgrade").is_none() && headers.get("Upgrade").unwrap() == "websocket")
        && (!headers.get("Connection").is_none()
            && (headers.get("Connection").unwrap() == "Upgrade"))
        && !headers.get("Sec-WebSocket-Key").is_none()
        && (!headers.get("Sec-WebSocket-Version").is_none()
            && headers.get("Sec-WebSocket-Version").unwrap() == "13")
    {
        // handle_websockets_connection::<TcpStream>(&headers, &mut writer);
        //one IP address per ws
        let mut writer = BufWriter::new(&mut stream);
        let status_line = "HTTP/1.1 101 Switching Protocols";
        let upgrade = "Upgrade: websocket";
        let connection = "Connection: Upgrade";
        let key = get_websocket_hash(headers.get("Sec-WebSocket-Key").unwrap());
        let ws_accept = format!("{}{}", "Sec-WebSocket-Accept: ", key);
        let response = format!(
            "{}\r\n{}\r\n{}\r\n{}\r\n\r\n",
            status_line, upgrade, connection, ws_accept
        );
        match writer.write(response.as_bytes()).await {
            Err(e) => {
                eprintln!("Could not write buffer into writer. Error: {}", e);
            }
            _ => {}
        };
        match writer.flush().await {
            Err(e) => {
                eprintln!("Could not flush output stream. Error: {}", e);
            }
            _ => {}
        };
        // then persist the websockets connection
        loop {
            //for now send to the client a simple hello
            let mut buf = BytesMut::new();
            buf.put_u8(0x81);
            buf.put_u8(0x05);
            buf.put_u8(0x48);
            buf.put_u8(0x65);
            buf.put_u8(0x6c);
            buf.put_u8(0x6c);
            buf.put_u8(0x6f);

            match writer.write(&buf).await {
                Err(e) => {
                    eprintln!("Could not write buffer into writer. Error: {}", e);
                }
                _ => {}
            };
            match writer.flush().await {
                Err(e) => {
                    eprintln!("Could not flush output stream. Error: {}", e);
                }
                _ => {}
            };
        }
    } else {
        let mut writer = BufWriter::new(&mut stream);
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
            Err(_) => (
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

        match writer.write(response.as_bytes()).await {
            Err(e) => {
                eprintln!("Could not write buffer into writer. Error: {}", e);
            }
            _ => {}
        };
        match writer.flush().await {
            Err(e) => {
                eprintln!("Could not flush output stream. Error: {}", e);
            }
            _ => {}
        };
    }
}
//TODO: refactor same code

async fn handle_websockets_connection<W: AsyncWrite>(
    headers: &HashMap<String, String>,
    writer: &mut BufWriter<W>,
) {
    //one IP address per ws
    let status_line = "HTTP/1.1 101 Switching Protocols";
    let upgrade = "Upgrade: websocket";
    let connection = "Connection: Upgrade";
    let key = get_websocket_hash(headers.get("Sec-WebSocket-Key").unwrap());
    let ws_accept = format!("{}{}", "Sec-WebSocket-Accept: ", key);
    let response = format!(
        "{}\r\n{}\r\n{}\r\n{}\r\n\r\n",
        status_line, upgrade, connection, ws_accept
    );
    // writer.write(response.as_bytes());
    // writer.flush();
}

fn get_websocket_hash(key: &String) -> String {
    let magic = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let mut key2 = key.clone();
    key2.push_str(magic);

    // create a Sha1 object
    let mut hasher = Sha1::new();
    // process input message
    hasher.update(key2);

    // acquire hash digest in the form of GenericArray,
    let result = hasher.finalize();
    encode(result)
}

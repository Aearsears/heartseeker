use std::collections::HashMap;
const REQ_LINE: &[&str] = &["Verb", "URI", "Version"];
const RES_LINE: &[&str] = &["Version", "Status", "Reason"];
pub enum Transactions {
    Req,
    Res,
}
/// takes in the HTTP transaction and returns
/// a hashmap of header -> value
pub fn parse_message(message: &String, sort: Transactions) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    let lines = message.lines();

    for line in lines {
        let part = line.split_once(":");
        match part {
            Some(d) => {
                hashmap.insert(d.0.to_string(), d.1.trim().to_string());
            }
            None => {
                let reqline = line.splitn(3, " ");
                for (i, part) in reqline.enumerate() {
                    match sort {
                        Transactions::Req => {
                            if !part.is_empty() {
                                hashmap.insert(REQ_LINE[i].to_string(), part.to_string());
                            }
                        }
                        Transactions::Res => {
                            if !part.is_empty() {
                                hashmap.insert(RES_LINE[i].to_string(), part.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    hashmap
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request_response_version() {
        let req = "HTTP/1.0 200 OK\r\nDate: Fri, 31 Dec 1999 23:59:59 GMT\r\nContent-Type: text/html\r\nContent-Length: 1354\r\n\r\n".to_string();
        let hashmap = parse_message(&req, Transactions::Res);
        assert_eq!(&"HTTP/1.0".to_string(), hashmap.get("Version").unwrap());
    }

    #[test]
    fn test_parse_request_response_status() {
        let req = "HTTP/1.0 200 OK\r\nDate: Fri, 31 Dec 1999 23:59:59 GMT\r\nContent-Type: text/html\r\nContent-Length: 1354\r\n\r\n".to_string();
        let hashmap = parse_message(&req, Transactions::Res);
        assert_eq!(&"200".to_string(), hashmap.get("Status").unwrap());
    }

    #[test]
    fn test_parse_request_response_reason() {
        let req = "HTTP/1.0 200 OK\r\nDate: Fri, 31 Dec 1999 23:59:59 GMT\r\nContent-Type: text/html\r\nContent-Length: 1354\r\n\r\n".to_string();
        let hashmap = parse_message(&req, Transactions::Res);
        assert_eq!(&"OK".to_string(), hashmap.get("Reason").unwrap());
    }

    #[test]
    fn test_parse_request_response_reason_2() {
        let req = "HTTP/1.0 404 Not Found\r\nDate: Fri, 31 Dec 1999 23:59:59 GMT\r\nContent-Type: text/html\r\nContent-Length: 1354\r\n\r\n".to_string();
        let hashmap = parse_message(&req, Transactions::Res);
        println!("{}", hashmap.get("Reason").unwrap());
        assert_eq!(&"Not Found".to_string(), hashmap.get("Reason").unwrap());
    }

    #[test]
    fn test_parse_request_response_header() {
        let req = "HTTP/1.0 200 OK\r\nDate: Fri, 31 Dec 1999 23:59:59 GMT\r\nContent-Type: text/html\r\nContent-Length: 1354\r\n\r\n".to_string();
        let hashmap = parse_message(&req, Transactions::Res);
        assert_eq!(&"1354".to_string(), hashmap.get("Content-Length").unwrap());
    }

    #[test]
    fn test_parse_request_request_verb() {
        let req = "GET /chat HTTP/1.1\r\nHost: server.example.com\r\n\r\n".to_string();
        let hashmap = parse_message(&req, Transactions::Req);
        assert_eq!(&"GET".to_string(), hashmap.get("Verb").unwrap());
    }

    #[test]
    fn test_parse_request_request_uri() {
        let req = "GET /chat HTTP/1.1\r\nHost: server.example.com\r\n\r\n".to_string();
        let hashmap = parse_message(&req, Transactions::Req);
        assert_eq!(&"/chat".to_string(), hashmap.get("URI").unwrap());
    }
    #[test]
    fn test_parse_request_request_version() {
        let req = "GET /chat HTTP/1.1\r\nHost: server.example.com\r\n\r\n".to_string();
        let hashmap = parse_message(&req, Transactions::Req);
        assert_eq!(&"HTTP/1.1".to_string(), hashmap.get("Version").unwrap());
    }

    #[test]
    fn test_parse_request_request_header() {
        let req = "GET /chat HTTP/1.1\r\nHost: server.example.com\r\n\r\n".to_string();
        let hashmap = parse_message(&req, Transactions::Req);
        assert_eq!(
            &"server.example.com".to_string(),
            hashmap.get("Host").unwrap()
        );
    }
}

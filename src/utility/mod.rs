use std::collections::HashMap;
/// takes in the HTTP initial line and header lines and returns
/// a hashmap of header -> value
pub fn get_headers(headers: &String) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    let lines = headers.lines();

    for line in lines {
        let part = line.split_once(":");
        match part {
            Some(d) => {
                hashmap.insert(d.0.to_string(), d.1.trim().to_string());
            }
            None => {}
        }
    }
    hashmap
}

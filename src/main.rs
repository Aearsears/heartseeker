use std::collections::HashMap;
use std::env;

mod gui;
mod io;
mod proxy;
mod threadpool;
mod webserver;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let pool = threadpool::ThreadPool::new(2);
    println!("Backend started...");
    io::log_to_file("Backend started...".to_string());

    pool.execute(|| {
        proxy::start_proxy(String::from("127.0.0.1:4000"));
    });
    pool.execute(|| webserver::start_admin_page(String::from("127.0.0.1:4001")));
    // let x = "HTTP/1.0 200 OK\r\nDate: Fri, 31 Dec 1999 23:59:59 GMT\r\nContent-Type: text/html\r\nContent-Length: 1354\r\n\r\n".to_string();
    // let mut lines = x.lines();

    // let mut hashmap: HashMap<String, String> = HashMap::new();

    // for line in lines {
    //     let part = line.split_once(":");
    //     match part {
    //         Some(d) => {
    //             hashmap.insert(d.0.to_string(), d.1.trim().to_string());
    //         }
    //         None => {}
    //     }
    // }

    // for (key, val) in hashmap.iter() {
    //     println!("key:{} val:{}", key, val);
    // }
}

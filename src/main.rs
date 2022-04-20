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
}

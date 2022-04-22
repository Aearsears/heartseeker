use std::env;

mod gui;
mod io;
mod server;
mod threadpool;
mod utility;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let pool = threadpool::ThreadPool::new(2);
    println!("Backend started...");
    io::log_to_file("Backend started...".to_string());

    pool.execute(|| {
        server::proxy::start_proxy(String::from("127.0.0.1:4000"));
    });
    pool.execute(|| server::webserver::start_admin_page(String::from("127.0.0.1:4001")));
}

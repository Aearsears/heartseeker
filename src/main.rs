use std::env;
use std::thread;

mod gui;
mod io;
mod server;
mod utility;

#[tokio::main]
async fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    println!("Backend started...");
    thread::spawn(|| {
        io::log_to_file("Backend started...".to_string());
    })
    .join()
    .expect("Could not write to logs.");

    thread::spawn(|| {
        server::webserver::start_admin_page(String::from("127.0.0.1:4001"));
    });

    thread::spawn(|| {
        server::proxy::start_proxy(String::from("127.0.0.1:4000"));
    });
    loop {}
}

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
mod gui;
mod proxy;
mod threadpool;
mod webserver;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let pool = threadpool::ThreadPool::new(2);
    println!("Backend started...");
    logging("Backend started...".to_string());
    pool.execute(|| {
        proxy::start_proxy(String::from("127.0.0.1:4000"));
    });
    pool.execute(|| webserver::start_admin_page(String::from("127.0.0.1:4001")));
}
// FIX permissions error
fn logging(msg: String) {
    let path = Path::new("logging.txt");
    let pathdisplay = path.display();
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            println!("Could not open {}, creating a new copy...", pathdisplay);
            let mut newfile = match File::create(&path) {
                Ok(file) => file,
                Err(why) => panic!(
                    "Could not create {} because of error {} aborting...",
                    pathdisplay, why
                ),
            };
            match newfile.write_all(format!("[{:?}]:{}", since_the_epoch, msg).as_bytes()) {
                Ok(_) => println!("Logged to {}", pathdisplay),
                Err(why) => println!("Could not write to {}. Error: {}", pathdisplay, why),
            };
            return;
        }
    };
    match file.write_all(format!("[{:?}]:{}", since_the_epoch, msg).as_bytes()) {
        Ok(_) => println!("Logged to {}", pathdisplay),
        Err(why) => println!("Could not write to {}. Error: {}", pathdisplay, why),
    };
}

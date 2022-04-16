use chrono::{TimeZone, Utc};
use std::env;
use std::io::Write;
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

fn logging(msg: String) {
    let path = Path::new("logging.txt");
    let pathdisplay = path.display();
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let date = Utc.timestamp(since_the_epoch.as_secs().try_into().unwrap(), 0);

    let mut file = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
    {
        Ok(file) => file,
        Err(why) => match why.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found.");
            }
            std::io::ErrorKind::PermissionDenied => {
                panic!("You don't have the sufficient permissions to open the file.");
            }
            _ => {
                panic!(
                    "Something went wrong while trying to open {} : {}",
                    pathdisplay, why
                );
            }
        },
    };
    match file.write_all(format!("[{}]:{}\r\n", date.to_rfc2822(), msg).as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Couldn't write to file: {}", e);
        }
    };
}

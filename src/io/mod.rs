use chrono::{TimeZone, Utc};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const LOGGING_PATH: &str = "./data/logging.txt";
const INFLOW_OUTFLOW_DATA_PATH: &str = "./data/inflow_outflow.json";

pub fn log_to_file(msg: String) {
    let path = Path::new(LOGGING_PATH);
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let date = Utc.timestamp(since_the_epoch.as_secs().try_into().unwrap(), 0);
    let formatted_date = date.to_rfc2822();
    let formatted_msg = format!("[{}]:{}\r\n", formatted_date, msg);
    write_to_file(&path, formatted_msg.as_bytes());
}

pub fn inflow_outflow_to_file(req: String) {
    let path = Path::new(INFLOW_OUTFLOW_DATA_PATH);
    write_to_file(&path, req.as_bytes());
}

fn write_to_file(path: &Path, msg: &[u8]) {
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
                    path.display(),
                    why
                );
            }
        },
    };
    match file.write_all(msg) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Couldn't write to file: {}", e);
        }
    };
}

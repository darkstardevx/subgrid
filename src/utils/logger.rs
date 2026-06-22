use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

pub fn log(message: &str) {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let log_entry = format!("[{}] - {}\n", timestamp, message);
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/opt/subgrid/logs/subgrid.log")
        .expect("Unable to open log file");
        
    file.write_all(log_entry.as_bytes()).expect("Unable to write data");
}

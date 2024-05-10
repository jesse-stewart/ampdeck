use simplelog::*;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use log::error;
extern crate libc;
use std::os::unix::io::AsRawFd;
use libc::{dup2, STDERR_FILENO};

pub fn setup_logging() -> Result<(), log::SetLoggerError> {
    // make sure ./logs directory exists
    std::fs::create_dir_all("./logs").unwrap();
    let log_file = File::create("./logs/log.log").unwrap();
    CombinedLogger::init(vec![
        WriteLogger::new(log::LevelFilter::Warn, Config::default(), log_file),
    ])
}

struct LoggerWriter;

impl Write for LoggerWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let msg = String::from_utf8_lossy(buf);
        error!("ALSA Output: {}", msg);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn redirect_stdout_stderr() {
    std::fs::create_dir_all("./logs").unwrap();
    let log_file = OpenOptions::new().create(true).append(true).open("./logs/log_sys.log").unwrap();
    let fd = log_file.as_raw_fd();

    unsafe {
        // Duplicate the log file fd to stdout
        // dup2(fd, STDOUT_FILENO);
        
        // Duplicate the log file fd to stderr
        dup2(fd, STDERR_FILENO);
    }
}
pub fn get_logs_entries() -> String {
    let log_file = File::open("./logs/log.log").unwrap();
    let mut reader = io::BufReader::new(log_file);
    let mut logs = String::new();
    reader.read_to_string(&mut logs).unwrap();
    logs = logs.lines().rev().take(50).collect::<Vec<&str>>().join("\n");
    logs
}

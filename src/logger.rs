use simplelog::*;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use log::error;
extern crate libc;
use std::os::unix::io::AsRawFd;
use libc::{dup2, STDOUT_FILENO};

pub fn setup_logging() -> Result<(), log::SetLoggerError> {
    let log_file = File::create("log.log").unwrap();
    CombinedLogger::init(vec![
        WriteLogger::new(log::LevelFilter::Info, Config::default(), log_file),
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
    let log_file = OpenOptions::new().create(true).append(true).open("log_sys.log").unwrap();
    let fd = log_file.as_raw_fd();

    unsafe {
        // Duplicate the log file fd to stdout
        dup2(fd, STDOUT_FILENO);
        
        // Duplicate the log file fd to stderr
        // dup2(fd, STDERR_FILENO);
    }
}


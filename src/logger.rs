use simplelog::*;
use std::fs::File;

pub fn setup_logging() -> Result<(), log::SetLoggerError> {
    let log_file = File::create("error_log.log").unwrap();
    CombinedLogger::init(vec![
        WriteLogger::new(log::LevelFilter::Error, Config::default(), log_file),
    ])
}

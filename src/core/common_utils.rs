use log::{info};
use std::io;
use std::path::Path;



/// Log channel variants
pub enum LogChannel { Info, Error }


/// Write error to log file
pub fn write_err_log(msg: &str, log_channel: LogChannel) {
    match log_channel {
        LogChannel::Info => info!("{}", msg),
        LogChannel::Error => info!("{}", msg)
    };
}


/// Build dyn path from component
pub fn build_path(basedir: &str, filename: &str) -> String {
    Path::new(basedir).join(filename).to_str().unwrap().to_string()
}


/// Convert any result to dummy Error
pub fn get_dummy_error<T, V>(_err: T) -> Result<V, io::Error> {
    Err(io::Error::from(io::ErrorKind::BrokenPipe))
}

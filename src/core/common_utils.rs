use std::path::Path;

use log::info;




/** Log channel variants */
pub enum LogChannel {
    Info,
    Error
}


/** Write error to log file */
pub fn write_err_log(msg: &str, log_channel: LogChannel) {
    match log_channel {
        LogChannel::Info => info!("{}", msg),
        LogChannel::Error => info!("{}", msg)
    };
}


/** Build dyn path from component */
pub fn build_path(basedir: &str, filename: &str) -> String {
    Path::new( basedir ).join( filename ).to_str().unwrap().to_string()
}

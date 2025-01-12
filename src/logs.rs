// ideas :
// - log to file from the config file path
// - create a specific thread at the start of the program to log to file
// - pass the thread as an app state

use std::fs::OpenOptions;
use std::io::Write;
use tokio;

#[derive(Clone)]
pub(crate) struct Logger {
    file: std::path::PathBuf,
    log_level: u8,
}

#[allow(unused)]
impl Logger {
    pub(crate) fn new(file_path: &str, log_level: u8) -> Logger {
        let file = std::path::PathBuf::from(file_path);

        Logger { file, log_level }
    }

    pub(crate) fn log(&mut self, msg: &str) {
        let logger = self.clone();
        // Move the thread away from the function to correct msg lifetime error
        tokio::spawn(async move {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(logger.file);

            match file {
                Ok(mut file) => file.write_all(msg.as_bytes()).unwrap_or(()),
                Err(_) => {
                    return;
                }
            }
        });
    }
}

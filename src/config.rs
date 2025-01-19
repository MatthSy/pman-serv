use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use serde::{Deserialize};
use crate::logs::Logger;

#[derive(Deserialize, Clone)]
pub(crate) struct ServerConfig {
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) data_dir: Option<String>,
    pub(crate) api_key_file: Option<String>,
    pub(crate) log_file: Option<String>,
    pub(crate) log_level: Option<u8>,
}

pub(crate) fn load_config(config_dir : Option<String>) -> ServerConfig {
    let config_dir = config_dir.unwrap_or(String::from("config.toml"));

    let mut file = File::open(config_dir).expect("Error opening the config file : file not found");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Error trying to read config file");

    toml::from_str(&text).expect("Error parsing config file")
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            ip: "127.0.0.1".to_string(),
            port: 8000,
            data_dir: None,
            api_key_file: None,
            log_file: None,
            log_level: None,
        }
    }
}

impl ServerConfig {

    // Gets the config's data dir or a default "data"
    pub(crate) fn data_dir(&self) -> String {
        self.data_dir.clone().unwrap_or(String::from("data"))
    }
    pub(crate) fn log_file(&self) -> String {
        self.log_file.clone().unwrap_or(String::from(""))
    }
    pub(crate) fn log_level(&self) -> u8 {
        self.log_level.unwrap_or(1)
    }

    pub(crate) fn new_logger(&self) -> Arc<Mutex<Logger>> {
        Arc::new(
            Mutex::new(
                Logger::new(&self.log_file(), self.log_level())
            )
        )
    }
}
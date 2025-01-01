use std::fs::File;
use std::io::Read;
use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub(crate) struct ServerConfig {
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) data_dir: Option<String>,
    pub(crate) api_key_file: Option<String>,
}

pub(crate) fn load_config(config_dir : Option<String>) -> ServerConfig {
    let config_dir = config_dir.unwrap_or(String::from("config.toml"));

    let mut file = File::open(config_dir).expect("Error opening the config file : file not found");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Error trying to read config file");

    toml::from_str(&*text).expect("Error parsing config file")
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            ip: "127.0.0.1".to_string(),
            port: 8000,
            data_dir: None,
            api_key_file: None,
        }
    }
}

impl ServerConfig {

    // Gets the config's data dir or a default "data"
    pub(crate) fn data_dir(&self) -> String {
        self.data_dir.clone().unwrap_or(String::from("data"))
    }
}
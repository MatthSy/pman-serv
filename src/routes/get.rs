use rocket::{get, State};
use std::fs::File;
use std::io::Read;
use crate::config::ServerConfig;

#[get("/")]
pub(crate) async fn index() {}

#[get("/passwords")]
pub(crate) async fn all_passwords_id(config: &State<ServerConfig>) -> Result<Vec<u8>, GetPassError> {
    // Get a list of all files in the data directory as an iterator
    let files = std::fs::read_dir(
        // Gets the config's data dir or a default "data"
        config.data_dir.clone().unwrap_or(String::from("data"))
    );
    if files.is_err() {
        return Err(GetPassError {
            message: "Error reading data directory".to_string(),
        });
    }

    // Read the files names and add it to the result vector
    let files = files.unwrap();
    let mut res : Vec<u8> = Vec::new();
    for file in files {
        let file = file.unwrap();
        let mut file_name = file.file_name().into_encoded_bytes();
        if res.len() > 0 { res.push('\n' as u8); }
        res.append(&mut file_name);
    }

    Ok(res)
}

#[get("/passwords/<password_id>")]
pub(crate) async fn password(password_id: &str) -> Result<Vec<u8>, GetPassError> {
    let file = File::open(format!("data/{}", password_id));
    if file.is_err() {
        return Err(GetPassError {
            message: "Password not found or other internal error".to_string(),
        });
    }
    let mut file = file.unwrap();
    let mut res = Vec::new();
    let read_result = file.read_to_end(&mut res);
    if read_result.is_err() {
        return Err(GetPassError {
            message: "Error reading password file".to_string(),
        });
    }
    Ok(res)
}

#[derive(Responder)]
struct GetPassError {
    message: String,
}
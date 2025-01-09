use std::fmt::format;
use rocket::{get, State};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use crate::api_keys::{ApiKey, ApiKeyStore};
use crate::config::ServerConfig;


#[derive(Responder)]
pub(crate) enum GetPasswordError {
    #[response(status = 500)]
    DirectoryErr(String),

}

#[allow(unused)]
#[get("/")]
pub(crate) async fn index(config: &State<ServerConfig>, api_key: &State<Arc<Mutex<ApiKeyStore>>>) -> Result<String, Err> {
    // TODO : renvoyer des info et des chiffres du serv

    let mut msg = String::new();

    let dir = std::fs::read_dir(config.data_dir());
    if dir.is_err() { msg += "Could not read directory\n"; }
    msg = format!("{}{}\n", msg, dir.unwrap().fold(0, |acc, _| acc + 1).to_string());



    // TODO créer des logs
}

#[get("/passwords")]
pub(crate) async fn all_passwords_id(config: &State<ServerConfig>, _api_key: &State<Arc<Mutex<ApiKeyStore>>>) -> Result<Vec<u8>, GetPasswordError> {
    // Get a list of all files in the data directory as an iterator
    let files = std::fs::read_dir(config.data_dir());
    if files.is_err() {
        return Err(GetPasswordError::DirectoryErr(String::from("Could not read directory")));
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
pub(crate) async fn password(config: &State<ServerConfig>, password_id: &str, _api_key: &State<Arc<Mutex<ApiKeyStore>>>) -> Result<Vec<u8>, GetPasswordError> {
    let file = File::open(format!("{}/{}", config.data_dir(), password_id));
    if file.is_err() {
        return Err(GetPasswordError::DirectoryErr(String::from("Password not found or other internal error")));
    }
    let mut file = file.unwrap();
    let mut res = Vec::new();
    let read_result = file.read_to_end(&mut res);
    if read_result.is_err() {
        return Err(GetPasswordError::DirectoryErr(String::from("Error reading password file")))
    }
    Ok(res)
}


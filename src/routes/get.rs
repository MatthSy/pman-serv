use rocket::{get, State};
use std::fs::{self, File};
use std::io::Read;
use std::sync::{Arc, Mutex};
use crate::api_keys::{ValidUser, ApiKeyStore};
use crate::config::ServerConfig;
use crate::input_guard::PostReqGuard;

#[derive(Responder)]
pub(crate) enum GetPasswordError {
    #[response(status = 500)]
    DirectoryErr(String),
}

#[allow(unused)]
#[get("/")]
pub(crate) async fn index(
    config: &State<ServerConfig>,
    api_key_store: &State<Arc<Mutex<ApiKeyStore>>>,
    api_user: ValidUser,
) -> Result<String, ()> {
    let mut msg = String::from("Server is running\n");

    // Trying to access (or create) user directory
    if let Ok(data_dir) = api_user.get_user_dir(&config) {

        // The number of passwords stored in user's directory:
        let dir = fs::read_dir(&data_dir);
        match dir {
            Ok(_) => {
                msg = format!(
                    "{}{}\n",
                    msg,
                    dir.unwrap().fold(0, |acc, _| acc + 1).to_string()
                );
            }
            Err(_) => {
                msg += "Could not read user directory\n";
            }
        }

        // The size of the data directory :
        if let Ok(dir_metadata) = std::fs::metadata(&data_dir) {
            msg = format!("{}{}\n", msg, dir_metadata.len());
        } else {
            msg += "Could not read directory\n";
        }

    } else {
        msg += "Could not get user directory\n";
    }

    // The number of API keys loaded :
    let api_keys = api_key_store.lock();
    if api_keys.is_err() {
        msg += "Unable to access API keys\n";
    } else {
        msg = format!("{}{}\n", msg, api_keys.unwrap().len());
    }

    Ok(msg)
}

#[get("/passwords")]
pub(crate) async fn all_passwords_id(config: &State<ServerConfig>, api_user: ValidUser, ) -> Result<Vec<u8>, GetPasswordError> {
    // Get a list of all files in the data directory as an iterator
    let dir = api_user.get_user_dir(&config);
    if dir.is_err() {
        return Err(GetPasswordError::DirectoryErr(String::from(
            "Could not access user directory",
        )));
    }

    let files = fs::read_dir(dir.unwrap());
    if files.is_err() {
        return Err(GetPasswordError::DirectoryErr(String::from(
            "Could not read user directory",
        )));
    }

    // Read the files names and add it to the result vector
    let files = files.unwrap();
    let mut res: Vec<u8> = Vec::new();

    for file in files {
        let file = file.unwrap();
        let mut file_name = file.file_name().into_encoded_bytes();
        if res.len() > 0 { res.push(b'\n' as u8); }
        res.append(&mut file_name);
    }

    Ok(res)
}

#[get("/passwords/<password_id>")]
pub(crate) async fn password(
    config: &State<ServerConfig>,
    password_id: &str,
    api_user: ValidUser,
) -> Result<Vec<u8>, GetPasswordError> {
    let dir = api_user.get_user_dir(&config);
    if dir.is_err() {
        return Err(GetPasswordError::DirectoryErr(String::from(
            "Could not access user directory",
        )));
    }

    let file = File::open(format!("{}/{}", dir.unwrap(), password_id));
    if file.is_err() {
        return Err(GetPasswordError::DirectoryErr(String::from(
            "Password not found or other internal error",
        )));
    }

    let mut file = file.unwrap();
    let mut res = Vec::new();
    let read_result = file.read_to_end(&mut res);
    if read_result.is_err() {
        return Err(GetPasswordError::DirectoryErr(String::from(
            "Error reading password file",
        )));
    }
    Ok(res)
}
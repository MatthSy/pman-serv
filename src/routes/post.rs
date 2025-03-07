use rocket::{post, State};
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::api_keys::{ValidUser, ApiKeyStore};
use crate::config::ServerConfig;
use crate::input_guard::PostReqGuard;

#[derive(Responder)]
pub(crate) enum PostError {
    #[response(status = 500)]
    FileError (String),

    #[response(status = 500)]
    OtherError (String),
}

#[post("/passwords/<password_id>", data = "<data>")]
pub(crate) async fn password(config: &State<ServerConfig>, password_id: &str, data: PostReqGuard, api_user: ValidUser) -> Result<(), PostError> {
    let dir = api_user.get_user_dir(&config);
    if dir.is_err() {
        return Err(PostError::FileError(String::from(
            "Could not access user directory",
        )));
    }

    let file = File::options()
        .write(true).create(true)
        .open(format!("{}/{}", dir.unwrap(), password_id));

    if file.is_err() {
        return Err(PostError::FileError(String::from("Error while opening or creating file")));
    }
    let mut file = file.unwrap();
    if file.write_all(data.0.as_ref()).is_err() {
        return Err(PostError::FileError(String::from("Error while writing to file")));
    }
    Ok(())
}

#[post("/api_keys/reload")]
pub(crate) async fn reload_api_keys(config: &State<ServerConfig>, valid_api_keys: &State<Arc<Mutex<ApiKeyStore>>>) -> Result<(), PostError> {
    if config.api_key_file.is_none() {
        return Err(PostError::FileError(String::from("No api key file provided in the server config")));
    }
    let mut valid_api_keys = match valid_api_keys.lock() {
        Ok(keys) => keys,
        Err(_) => return Err(PostError::OtherError(String::from("Error while locking the api keys list"))),
    };
    valid_api_keys.load_keys(config.api_key_file.clone().unwrap());
    Ok(())
}
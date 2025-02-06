use std::fs;
use std::io::{Read, Write};
use rocket::{delete, State};
use crate::api_keys::ValidUser;
use crate::config::ServerConfig;

#[allow(unused)]
#[derive(Responder)]
pub(crate) enum DeletePasswordError {
    #[response(status = 500)]
    DirectoryErr(String),
    #[response(status = 400)]
    PasswordErr(String),
}

#[delete("/passwords/<password_id>")]
pub(crate) async fn delete_password(password_id: &str, config: &State<ServerConfig>, api_user: ValidUser) -> Result<(), DeletePasswordError> {
    let dir = api_user.get_user_dir(&config);
    if dir.is_err() {
        return Err(DeletePasswordError::DirectoryErr(String::from(
            "Could not access user directory",
        )));
    }

    // Check if password exists
    let file = fs::File::open(format!("{}/{}", dir.clone().unwrap(), password_id));
    if let Err(_) = file {
        return Err(DeletePasswordError::PasswordErr(String::from("Password not found")));
    }


    // Backup password in a backup subdirectory :
    // First create backup directory if it does not exist
    fs::create_dir(format!("{}/backup/", dir.clone().unwrap())).ok();
    // Then create backup file and copy text to it via a buffer
    let mut backup_file = fs::OpenOptions::new().write(true).create(true).open(
    format!("{}/backup/{}", dir.clone().unwrap(), password_id)
    ).unwrap();
    let mut buf = String::new();
    if let Err(_) = file.unwrap().read_to_string(&mut buf) {
        return Err(DeletePasswordError::DirectoryErr(String::from(
            "Could not read password file",
        )));
    }
    backup_file.write_all(buf.as_ref()).unwrap();


    // Try to delete password
    if let Err(_) = fs::remove_file(format!("{}/{}", dir.clone().unwrap(), password_id)) {
        return Err(DeletePasswordError::PasswordErr(String::from("Could not delete password")));
    }

    Ok(())
}
use std::fs;
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
    if let Err(_) = fs::File::open(format!("{}/{}", dir.clone().unwrap(), password_id)) {
        return Err(DeletePasswordError::PasswordErr(String::from("Password not found")));
    }

    // Try to delete password
    // TODO: copy the password to a backup file before deleting
    if let Err(_) = fs::remove_file(format!("{}/{}", dir.clone().unwrap(), password_id)) {
        return Err(DeletePasswordError::PasswordErr(String::from("Could not delete password")));
    }

    Ok(())
}
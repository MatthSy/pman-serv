use rocket::{post, State};
use std::fs::File;
use std::io::Write;
use crate::config::ServerConfig;


#[derive(Responder)]
pub(crate) enum PostError {
    FileError { message : String },
}

#[post("/passwords/<password_id>", data = "<data>")]
pub(crate) async fn password(config: &State<ServerConfig>, password_id: &str, data: &str) -> Result<(), PostError> {
    let file = File::options()
        .write(true).create(true)
        .open(format!("{}/{}", config.data_dir(), password_id));

    if file.is_err() {
        return Err(PostError::FileError{message: String::from("Error while opening or creating file")});
    }
    let mut file = file.unwrap();
    if file.write_all(data.as_ref()).is_err() {
        return Err(PostError::FileError {message: String::from("Error while writing to file")});
    }
    Ok(())
}
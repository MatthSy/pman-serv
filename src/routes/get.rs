use std::fmt::format;
use rocket::get;
use rocket::sentinel::resolution::Resolve;
use std::fs::File;
use std::io::Read;

#[get("/")]
pub(crate) async fn index() {}

#[get("/passwords")]
pub(crate) async fn all_passwords_id() -> Result<Vec<String>, GetPassError> {
    let files = std::fs::read_dir("data");
    if files.is_err() {
        return Err(GetPassError {
            message: "Error reading data directory".to_string(),
        });
    }
    let files = files.unwrap();
    let mut res = Vec::new();
    for file in files {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        res.push(file_name.to_string());
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
use rocket::get;
use rocket::sentinel::resolution::Resolve;
use std::fs::File;
use std::io::Read;

#[get("/")]
pub(crate) async fn index() {
    println!("GET request on index");
}

#[get("/password/<password_id>")]
pub(crate) async fn password(password_id: &str) -> Result<Vec<u8>, GetPassError> {
    let file = File::open("data/".to_owned() + password_id);
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
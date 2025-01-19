use rocket::delete;

enum DeletePasswordError {
    #[response(status = 500)]
    DirectoryErr(String),
}

#[delete("/passwords/<password_id>")]
pub(crate) async fn delete_password(password_id: &str, ) -> Result<(), ()> {
    Ok(())
}
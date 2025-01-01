#[catch(401)]
pub(crate) fn unauthorized() -> String {
    String::from("Unauthorized, invalid API key")
}
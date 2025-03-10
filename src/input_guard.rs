use rocket::data::{FromData, Outcome};
use rocket::{Data, Request};
use tokio::io::AsyncReadExt;
use pmanApiLib::encryption::EncryptedData;

pub(crate) struct PostReqGuard(pub(crate) EncryptedData);


#[rocket::async_trait]
impl<'r> FromData<'r> for PostReqGuard {
    type Error = ();

    async fn from_data(_req: &'r Request<'_>, mut data: Data<'r>) -> Outcome<'r, Self> {
        match post_pattern_matching(&mut data).await {
            Some(val) => Outcome::Success(PostReqGuard(val)),
            None => Outcome::Error((rocket::http::Status::BadRequest, ())),
        }
    }
}

async fn post_pattern_matching(data: &mut Data<'_>) -> Option<EncryptedData> {
    let mut buf = String::new();
    data.peek(512)
        .await
        .read_to_string(&mut buf)
        .await
        .unwrap_or(0);

    toml::from_str::<EncryptedData>(&buf).ok()
}
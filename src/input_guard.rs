use rocket::data::{FromData, Outcome};
use rocket::{Data, Request};
use tokio::io::AsyncReadExt;

pub(crate) struct PostReqGuard(pub(crate) String);


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

async fn post_pattern_matching(data: &mut Data<'_>) -> Option<String> {
    let mut buf = String::new();
    data.peek(512)
        .await
        .read_to_string(&mut buf)
        .await
        .unwrap_or(0);
    dbg!(&buf);
    let mut lines = buf.lines();
    if let None = lines.next() {
        return None;
    }

    //
    // First line :
    let first_line = match lines.next() {
        Some(line) => line,
        None => return None,
    };
    let mut split = first_line.split(" = ");

    let s1 = match split.next() {
        Some(s) => s,
        None => return None,
    };
    match s1 {
        "password" => {}
        _ => return None,
    }

    let s2 = match split.next() {
        Some(s) => s,
        None => return None,
    };
    if let Ok(password) = hex::decode(s2) {
        if password.len() != 32 {
            return None;
        }
    } else {
        return None;
    }

    //
    // Second line :
    //
    let second_line = match lines.next() {
        Some(line) => line,
        None => return None,
    };
    let mut split = second_line.split(" = ");

    let s1 = match split.next() {
        Some(s) => s,
        None => return None,
    };
    match s1 {
        "id"=> {}
        _ => return None,
    }

    let s2 = split.next();
    if s2.is_none() {
        return None;
    }

    //
    // Third line :
    //
    let third_line = match lines.next() {
        Some(line) => line,
        None => return None,
    };
    let mut split = third_line.split(" = ");

    let s1 = match split.next() {
        Some(s) => s,
        None => return None,
    };
    match s1 {
        "tag" => {}
        _ => return None,
    }


    let s2 = match split.next() {
        Some(s) => s,
        None => return None,
    };
    if let Ok(password) = hex::decode(s2) {
        if password.len() != 16 {
            return None;
        }
    } else {
        return None;
    }

    // Ensure there is no line after
    match split.next() {
        Some("") => {}
        Some(_) => return None,
        None => {}
    }

    Some(buf)
}

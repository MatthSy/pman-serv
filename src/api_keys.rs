use crate::api_keys::ApiKeyError::{InvalidApiKey, InvalidUser};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub(crate) struct ApiKeyStore {
    keys: HashMap<String, String>,
}

impl ApiKeyStore {
    pub(crate) fn new() -> ApiKeyStore {
        ApiKeyStore {
            keys: HashMap::new(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.keys.len()
    }

    pub(crate) fn is_valid(&self, key: &str, user: &str) -> Result<(), ApiKeyError> {
        if !self.keys.contains_key(key) {
            return Err(InvalidApiKey);
        } else if self.keys.get(key).is_none() || self.keys.get(key).unwrap() != user {
            return Err(InvalidUser);
        }

        Ok(())
    }

    pub(crate) fn load_keys(&mut self, path: String) -> &mut Self {
        fs::read_to_string(path)
            .expect("Failed to read file")
            .lines()
            .for_each(|line| {
                let mut line = line.split(" ");
                self.keys
                    .insert(String::from(line.next().unwrap_or("")), String::from(line.next().unwrap_or("")));
            });
        self
    }

    pub(crate) fn from_file(path: String) -> ApiKeyStore {
        let mut store = ApiKeyStore::new();
        store.load_keys(path);
        store
    }
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct ApiKey(String);

#[derive(Debug)]
pub(crate) enum ApiKeyError {
    MissingApiKey,
    MissingUser,
    InvalidApiKey,
    InvalidUser,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key_list = request
            .guard::<&rocket::State<Arc<Mutex<ApiKeyStore>>>>()
            .await
            .unwrap();

        let api_key = request.headers().get_one("X-API-KEY");
        let api_user = request.headers().get_one("X-USER-NAME");

        if api_key.is_none() {
            return Outcome::Error((rocket::http::Status::BadRequest, ApiKeyError::MissingApiKey));
        }
        if api_user.is_none() {
            return Outcome::Error((rocket::http::Status::BadRequest, ApiKeyError::MissingUser));
        }

        if let Err(err) = api_key_list
            .lock()
            .unwrap()
            .is_valid(api_key.unwrap(), api_user.unwrap())
        {
            Outcome::Error((rocket::http::Status::BadRequest, err))
        } else {
            Outcome::Success(ApiKey(String::from(api_key.unwrap())))
        }
    }
}

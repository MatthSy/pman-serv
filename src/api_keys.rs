use std::collections::HashSet;
use std::fs;
use std::sync::{Arc, Mutex};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

#[derive(Debug)]
pub(crate) struct ApiKeyStore {
    keys: HashSet<String>,
}

impl ApiKeyStore {
    pub(crate) fn new() -> ApiKeyStore {
        ApiKeyStore {
            keys: HashSet::new(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.keys.len()
    }

    pub(crate) fn is_valid(&self, key: &str) -> bool {
        self.keys.contains(key)
    }

    pub(crate) fn load_keys(&mut self, path: String) -> &mut Self {
        fs::read_to_string(path)
            .expect("Failed to read file")
            .lines()
            .for_each( |key| {
                    self.keys.insert(String::from(key));
                });
        self
    }

    pub(crate) fn from_file(path: String) -> ApiKeyStore {
        let mut store = ApiKeyStore::new();
        store.load_keys(path);
        store
    }
}

#[derive(Debug)]
pub(crate) struct ApiKey(String);

#[derive(Debug)]
pub(crate) enum ApiKeyError {
    MissingApiKey,
    InvalidApiKey,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key_list = request.guard::<&rocket::State<Arc<Mutex<ApiKeyStore>>>>().await.unwrap();
        if let Some(key) = request.headers().get_one("X-API-KEY") {
            return if api_key_list.lock().unwrap().is_valid(key) {
                Outcome::Success(ApiKey(String::from(key)))
            } else {
                Outcome::Error((rocket::http::Status::Unauthorized, ApiKeyError::InvalidApiKey))
            }
        }
        Outcome::Error((rocket::http::Status::BadRequest, ApiKeyError::MissingApiKey))
    }
}
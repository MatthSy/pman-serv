use crate::api_keys::ApiKeyError::{InvalidApiKey, InvalidUser};
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use crate::config::ServerConfig;

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct ApiKeyStore {
    keys: HashMap<String, String>,
}

#[allow(unused)]
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
        let read_data = fs::read_to_string(&path);
            let valid_data = read_data.unwrap();
            valid_data.lines()
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
pub(crate) struct ValidUser {
    key: String,
    user_name: String,
}

#[allow(unused)]
impl ValidUser {
    pub(crate) fn key(&self) -> &str {
        &self.key
    }

    pub(crate) fn user_name(&self) -> &str {
        &self.user_name
    }

    pub(crate) fn get_user_dir(&self, config: &State<ServerConfig>) -> Result<String, ()> {
        let data_dir = config.data_dir() + &*format!("/{}", &self.user_name);

        if let Ok(_) = fs::DirBuilder::new().recursive(true).create(&data_dir) {
            Ok(data_dir)
        } else {
            Err(())
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) enum ApiKeyError {
    MissingApiKey,
    MissingUser,
    InvalidApiKey,
    InvalidUser,
    ReservedUserName,
}

#[allow(unused)]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ValidUser {
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
        let api_key = api_key.unwrap();
        let api_user = api_user.unwrap();

        // Checks for reserved usernames
        if api_user.to_lowercase() == "backup" {
            return Outcome::Error((rocket::http::Status::BadRequest, ApiKeyError::ReservedUserName));
        }

        if let Err(err) = api_key_list
            .lock()
            .unwrap()
            .is_valid(api_key, api_user)
        {
            Outcome::Error((rocket::http::Status::BadRequest, err))
        } else {
            Outcome::Success(ValidUser {
                key: String::from(api_key),
                user_name: String::from(api_user),
            })
        }
    }
}

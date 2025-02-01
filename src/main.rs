mod routes;
mod config;
mod api_keys;
mod logs;

use crate::config::ServerConfig;
use crate::routes::{get, post, delete};
use crate::api_keys::ApiKeyStore;

use std::sync::{Arc, Mutex};

#[macro_use] extern crate rocket;
use rocket::Config;
use crate::logs::FairingLogger;

#[launch]
fn rocket() -> _ {
    let config: ServerConfig = config::load_config(None);
    let figment = Config::figment()
        .merge(("address", &config.ip))
        .merge(("port", &config.port));

    let key_store = ApiKeyStore::from_file(config.api_key_file.clone().unwrap());
    let valid_api_keys = Arc::new(Mutex::new(key_store));

    let logger = config.new_logger();

    rocket::build()
        .configure(figment) // The Rocket config
        .manage(config) // Sets the app server config as a Rocket State
        .manage(valid_api_keys) // Sets the api keys list as a Rocket State
        .manage(Arc::clone(&logger)) // Sets the logger as a Rocket State
        .attach(FairingLogger::new(logger)) // Attaches the logger fairing
        .mount("/", routes![get::index, get::password, get::all_passwords_id]) // Mounts the get routes
        .mount("/", routes![post::password, post::reload_api_keys]) // Mounts the post route
        .mount("/", routes![delete::delete_password]) // Mounts the delete route
        .register("/", catchers![routes::catchers::unauthorized]) // Mounts the  catchers
}

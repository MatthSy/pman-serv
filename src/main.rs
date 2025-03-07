mod api_keys;
mod config;
mod logs;
mod routes;
mod jobs;
mod input_guard;

use crate::api_keys::ApiKeyStore;
use crate::config::ServerConfig;
use crate::routes::{delete, get, post};

use std::sync::{Arc, Mutex};

#[macro_use]
extern crate rocket;
use crate::logs::FairingLogger;
use rocket::Config;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    // Load config
    let config: ServerConfig = config::load_config(None);
    let figment = Config::figment()
        .merge(("address", &config.ip))
        .merge(("port", &config.port));

    // Load api keys
    let key_store = ApiKeyStore::from_file(config.api_key_file.clone().unwrap());
    let valid_api_keys = Arc::new(Mutex::new(key_store));

    // The logger used through the app
    let logger = config.new_logger();

    // The job for removing backups after 2 weeks
    let data_dir = config.data_dir().clone();
    let tmp_logger = Arc::clone(&logger);
    tokio::spawn(async move {
        jobs::remove_backup_job(data_dir, tmp_logger).await
    });

    // The rocket app
    let r = rocket::build()
        .configure(figment) // The Rocket config
        .manage(config) // Sets the app server config as a Rocket State
        .manage(valid_api_keys) // Sets the api keys list as a Rocket State
        .manage(Arc::clone(&logger)) // Sets the logger as a Rocket State
        .attach(FairingLogger::new(logger)) // Attaches the logger fairing
        .mount(
            "/",
            routes![get::index, get::password, get::all_passwords_id],
        ) // Mounts the get routes
        .mount("/", routes![post::password, post::reload_api_keys]) // Mounts the post route
        .mount("/", routes![delete::delete_password]) // Mounts the delete route
        .register("/", catchers![routes::catchers::unauthorized]); // Mounts the  catchers

    // Starts the actual web app
    r.launch().await?;
    Ok(())
}

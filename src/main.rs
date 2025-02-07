mod api_keys;
mod config;
mod logs;
mod routes;

use crate::api_keys::ApiKeyStore;
use crate::config::ServerConfig;
use crate::routes::{delete, get, post};

use chrono::Utc;
use std::sync::{Arc, Mutex};
use tokio::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

#[macro_use]
extern crate rocket;
use crate::logs::FairingLogger;
use rocket::Config;

// #[launch]
// fn rocket() -> _ {
//     let config: ServerConfig = config::load_config(None);
//     let figment = Config::figment()
//         .merge(("address", &config.ip))
//         .merge(("port", &config.port));
//
//     let key_store = ApiKeyStore::from_file(config.api_key_file.clone().unwrap());
//     let valid_api_keys = Arc::new(Mutex::new(key_store));
//
//     let logger = config.new_logger();
//
//     tokio::spawn(async {
//         // Start the scheduler
//         let scheduler = JobScheduler::new().await.unwrap();
//
//         let job = Job::new_tz("0 0 2 * * * *", Utc, |_, _| { // Runs at 2 o'clock every day
//             tokio::spawn(async {
//                 // TODO : verify if every backup is older than a week and remove it definitely
//             });
//         }).unwrap();
//
//         scheduler.add(job).await.unwrap();
//
//         // Start the scheduler
//         scheduler.start().await.unwrap();
//
//         // Keep the main thread alive
//         loop {
//             tokio::time::sleep(Duration::from_secs(60)).await; // Sleep to keep the main thread alive
//         }
//     });
//
//
//     rocket::build()
//         .configure(figment) // The Rocket config
//         .manage(config) // Sets the app server config as a Rocket State
//         .manage(valid_api_keys) // Sets the api keys list as a Rocket State
//         .manage(Arc::clone(&logger)) // Sets the logger as a Rocket State
//         .attach(FairingLogger::new(logger)) // Attaches the logger fairing
//         .mount("/", routes![get::index, get::password, get::all_passwords_id]) // Mounts the get routes
//         .mount("/", routes![post::password, post::reload_api_keys]) // Mounts the post route
//         .mount("/", routes![delete::delete_password]) // Mounts the delete route
//         .register("/", catchers![routes::catchers::unauthorized]) // Mounts the  catchers
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config: ServerConfig = config::load_config(None);
    let figment = Config::figment()
        .merge(("address", &config.ip))
        .merge(("port", &config.port));

    let key_store = ApiKeyStore::from_file(config.api_key_file.clone().unwrap());
    let valid_api_keys = Arc::new(Mutex::new(key_store));

    let logger = config.new_logger();

    let data_dir = config.data_dir().clone();
    tokio::spawn(async move {
        // Start the scheduler
        let scheduler = JobScheduler::new().await.unwrap();

        // TODO : add logging for deletion of a password and running of the job
        let job = Job::new_tz("0 0 2 * * *", Utc, move |_, _| {
            // Runs at 2 o'clock every day
            let data_dir = data_dir.clone();
            tokio::spawn(async move {

                // If read data_dir is Ok, iterate over each
                if let Ok(user_directories) = std::fs::read_dir(data_dir) {
                    user_directories.for_each(|dir| {
                        // If read backup_dir is Ok, iterate over each
                        let mut backup_dir = dir.unwrap().path();
                        backup_dir.push("backup");

                        if let Ok(files) = std::fs::read_dir(backup_dir) {
                            files.for_each(|file| {
                                let file = file.unwrap();
                                let metadata = file.metadata().unwrap();
                                let duration = metadata.modified().unwrap().elapsed().unwrap();
                                if duration.as_secs() > 1 {
                                    // Seven days : 604800 secs
                                    std::fs::remove_file(file.path()).unwrap();
                                }
                            });
                        }
                    });
                }
            });
        })
        .unwrap();

        scheduler.add(job).await.unwrap();

        // Start the scheduler
        scheduler.start().await.unwrap();

        // Keep the main thread alive
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await; // Sleep to keep the main thread alive
            // println!("Alive");
        }
    });

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

    r.launch().await?;

    Ok(())
}

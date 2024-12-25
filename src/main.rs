mod routes;
mod config;

use rocket::Config;
use crate::config::ServerConfig;
use crate::routes::get;
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    let config: ServerConfig = config::load_config(None);
    let figment = Config::figment()
        .merge(("address", &config.ip))
        .merge(("port", &config.port));

    rocket::build()
        .configure(figment) // The Rocket config
        .manage(config) // Sets the app server config as a State
        .mount("/", routes![get::index, get::password, get::all_passwords_id]) // Mounts the get routes
}

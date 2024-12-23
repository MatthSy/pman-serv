mod routes;
mod config;

use crate::config::ServerConfig;
use crate::routes::get;
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    let config: ServerConfig = config::load_config(None);

    rocket::build()
        .manage(config)
        .mount("/", routes![get::index, get::password, get::all_passwords_id])
}

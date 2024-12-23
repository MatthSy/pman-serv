mod routes;
use crate::routes::get;
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get::index, get::password])
}

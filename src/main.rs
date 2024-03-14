#[macro_use] extern crate rocket;

mod routes;
mod services;

use routes::index::index;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

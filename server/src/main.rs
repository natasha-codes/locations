#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::Json;

mod location;
mod user;

use location::{Coordinate, Location};
use user::User;

fn main() {
    rocket::ignite()
        .mount("/", routes![get_a_location])
        .launch();
}

#[get("/hello")]
fn get_a_location(_user: User) -> Json<Location> {
    Json(Location {
        lat: Coordinate {
            hour: 1,
            min: 2,
            second: 3,
        },
        long: Coordinate {
            hour: 4,
            min: 5,
            second: 6,
        },
    })
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod location;

use location::{Coordinate, Location};
use rocket_contrib::json::Json;

fn main() {
    rocket::ignite()
        .mount("/", routes![get_a_location])
        .launch();
}

#[get("/hello")]
fn get_a_location() -> Json<Location> {
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

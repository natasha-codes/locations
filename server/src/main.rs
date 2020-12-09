#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod location;

use location::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![get_a_location])
        .launch();
}

#[get("/hello/<name>")]
fn get_a_location(name: String) -> String {
    let location = Location {
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
    };

    format!("Hello, {}. You are at: {}", name, location)
}

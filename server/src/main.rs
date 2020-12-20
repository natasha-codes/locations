#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};

mod location;
mod openid;
mod user;

use location::{Coordinate, Location};
use rocket_contrib::json::Json;
use user::User;

#[tokio::main]
async fn main() {
    rocket::ignite()
        .mount("/", routes![get_a_location])
        .launch()
        .await
        .expect("Rocket failed to launch");
}

#[get("/hello", format = "json")]
async fn get_a_location(user: User) -> Json<Location> {
    println!("{:?}", user.id());

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

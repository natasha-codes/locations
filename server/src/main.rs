#[macro_use]
extern crate rocket;

use rocket::{get, routes};

mod location;
mod openid;
mod user;

use location::{Coordinate, Location};
use rocket_contrib::json::Json;
use user::User;

#[launch]
async fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![get_a_location])
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

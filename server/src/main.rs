#![feature(proc_macro_hygiene, decl_macro)]

use rocket::get;
use rocket_contrib::json::Json;

mod location;
mod openid;
mod user;

use location::{Coordinate, Location};
use user::User;

#[tokio::main]
async fn main() {
    // let key_set = authentication::get_msa_key_set().await;
    // println!("{:?}", key_set);

    /*
    rocket::ignite()
        .mount("/", routes![get_a_location])
        .launch();
    */
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

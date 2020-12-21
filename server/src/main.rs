#[macro_use]
extern crate rocket;

use rocket::routes;
use rocket_contrib::json::Json;

mod openid;
mod storage;
mod user;

use openid::JwtValidator;
use user::User;

#[launch]
async fn rocket() -> rocket::Rocket {
    foo().await;

    rocket::ignite()
        .manage(JwtValidator::new_msa())
        .mount("/", routes![get_a_location])
}

async fn foo() {
    storage::db_client::DbClient::connect().await.expect("ahhh");
}

#[post("/hello")]
async fn get_a_location(user: User) -> Json<String> {
    Json(user.id().clone())
}

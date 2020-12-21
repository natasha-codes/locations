#[macro_use]
extern crate rocket;

use rocket::routes;
use rocket_contrib::json::Json;

mod auth;
mod storage;

use auth::{openid::JwtValidator, AuthenticatedUser};

#[launch]
async fn rocket() -> rocket::Rocket {
    foo().await;

    rocket::ignite()
        .manage(JwtValidator::new_msa())
        .mount("/", routes![get_a_location])
}

async fn foo() {
    storage::mongo_manager::MongoManager::new("mongodb://localhost:27017/")
        .await
        .expect("ahhh");
}

#[post("/hello")]
async fn get_a_location(user: AuthenticatedUser) -> Json<String> {
    Json(user.id().clone())
}

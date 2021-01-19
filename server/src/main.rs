#[macro_use]
extern crate rocket;

use rocket::{routes, State};
use rocket_contrib::json::Json;

mod auth;
mod models;
mod routes;
mod storage;

use auth::{openid::JwtValidator, AuthError, AuthenticatedUser};
use models::{api::Contact, common::Location};
use routes::{RouteResult, ToRouteResult};
use storage::MongoManager;

#[launch]
async fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(JwtValidator::new_msa())
        .manage(
            MongoManager::new("mongodb://localhost:27017")
                .await
                .expect("Failed to connect to Mongo"),
        )
        .mount("/", routes![get_my_location, upload_my_location])
}

#[post("/my/location", data = "<location>")]
async fn upload_my_location(
    user_auth: Result<AuthenticatedUser, AuthError>,
    mongo: State<'_, MongoManager>,
    location: Json<Location>,
) -> RouteResult<()> {
    // Early-returns if unable to auth the user.
    let my_user_id = user_auth?.id();

    mongo
        .update_user_location(&my_user_id, *location)
        .await
        .to_route_result()
}

#[get("/my/location")]
async fn get_my_location(
    user_auth: Result<AuthenticatedUser, AuthError>,
    mongo: State<'_, MongoManager>,
) -> RouteResult<Contact> {
    // Early-returns if unable to auth the user.
    let my_user_id = user_auth?.id();

    let my_user = mongo.get_user_by_id(&my_user_id).await?;

    Contact::from(my_user).to_route_result()
}

#[macro_use]
extern crate rocket;

use rocket::{routes, State};

mod auth;
mod models;
mod storage;

use auth::{openid::JwtValidator, AuthError, AuthenticatedUser};
use models::{
    api::{ApiError, Contact, Empty, IncomingModel, OutgoingModel},
    common::Location,
};
use storage::MongoManager;

type RouteResult<T> = std::result::Result<OutgoingModel<T>, ApiError>;

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
    location: IncomingModel<'_, Location>,
) -> RouteResult<Empty> {
    let my_user_id = user_auth?.id();

    mongo
        .update_user_location(&my_user_id, *location)
        .await
        .map(|void| void.into())
        .map_err(|err| err.into())
}

#[get("/my/location")]
async fn get_my_location(
    user_auth: Result<AuthenticatedUser, AuthError>,
    mongo: State<'_, MongoManager>,
) -> RouteResult<Contact> {
    // Early-returns if unable to auth the user.
    let my_user_id = user_auth?.id();

    let my_user = mongo.get_user_by_id(&my_user_id).await?;

    Ok(Contact::from(my_user).into())
}

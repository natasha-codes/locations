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
use rocket_contrib::json::Json;
use storage::MongoManager;

type MaybeRouteResult<T> = std::result::Result<Option<OutgoingModel<T>>, ApiError>;
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
    _user: Result<AuthenticatedUser, AuthError>,
    _mongo: State<'_, MongoManager>,
    location: IncomingModel<'_, Location>,
) -> RouteResult<Empty> {
    println!("Location: {:?}", *location);

    Ok(().into())
}

#[get("/my/location")]
async fn get_my_location(
    user: Result<AuthenticatedUser, AuthError>,
    mongo: State<'_, MongoManager>,
) -> MaybeRouteResult<Contact> {
    match mongo.get_user_by_id(user?.id()).await? {
        Some(user) => {
            println!("Found user: {:?}", user);

            Ok(Some(Contact::from(user).into()))
        }
        None => {
            println!("No user found!");

            Ok(None)
        }
    }
}

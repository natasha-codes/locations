#[macro_use]
extern crate rocket;

use rocket::{routes, State};

mod auth;
mod models;
mod storage;

use auth::{openid::JwtValidator, AuthError, AuthenticatedUser};
use models::api::{ApiError, Contact, OutgoingModel};
use storage::MongoManager;

type MaybeRouteResult<T> = std::result::Result<Option<OutgoingModel<T>>, ApiError>;
#[allow(dead_code)]
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
        .mount("/", routes![get_my_location])
}

#[get("/my/location")]
async fn get_my_location(
    user: Result<AuthenticatedUser, AuthError>,
    mongo: State<'_, MongoManager>,
) -> MaybeRouteResult<Contact> {
    match mongo.get_user_by_id(user?.id()).await? {
        Some(user) => Ok(Some(Contact::from(user).into())),
        None => Ok(None),
    }
}

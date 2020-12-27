#[macro_use]
extern crate rocket;

use rocket::{http::Status, routes, State};

mod auth;
mod models;
mod storage;

use auth::{openid::JwtValidator, AuthenticatedUser};
use models::api::{Contact, OutgoingModel};
use storage::MongoManager;

type MaybeRouteResult<T> = std::result::Result<Option<OutgoingModel<T>>, Status>;
#[allow(dead_code)]
type RouteResult<T> = std::result::Result<OutgoingModel<T>, Status>;

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

#[get("/my/<id>/location")]
async fn get_my_location(
    id: String,
    user: AuthenticatedUser,
    mongo: State<'_, MongoManager>,
) -> MaybeRouteResult<Contact> {
    if user.id() != &id {
        // Return `None`, i.e. a 404, if the user IDs don't match. Prefer this
        // to an auth error, so as not to leak user IDs.
        return Ok(None);
    }

    match mongo
        .get_user_by_id(&id)
        .await
        .map_err(|_| Status::InternalServerError)?
    {
        Some(user) => Ok(Some(Contact::from(user).into())),
        None => Ok(None),
    }
}

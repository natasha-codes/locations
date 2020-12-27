#[macro_use]
extern crate rocket;

use rocket::{http::Status, routes, State};

mod auth;
mod models;
mod storage;

use auth::{openid::JwtValidator, AuthenticatedUser};
use models::api::{Contact, ExternallyExposed, OutgoingModel};
use storage::MongoManager;

type MaybeRouteResult<T: ExternallyExposed> = std::result::Result<Option<OutgoingModel<T>>, Status>;
type RouteResult<T: ExternallyExposed> = std::result::Result<OutgoingModel<T>, Status>;

#[launch]
async fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(JwtValidator::new_msa())
        .manage(
            MongoManager::new("mongodb://localhost:27017")
                .await
                .expect("Failed to connect to Mongo"),
        )
        .mount("/", routes![refresh_my_contacts])
}

#[get("/my/<id>/contacts")]
async fn refresh_my_contacts(
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
        Some(user) => Ok(Some(Contact::new().into())),
        None => Ok(None),
    }
}

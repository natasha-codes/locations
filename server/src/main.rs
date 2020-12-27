#[macro_use]
extern crate rocket;

use rocket::{http::Status, routes, State};
use rocket_contrib::json::Json;

mod auth;
mod models;
mod storage;

use auth::{openid::JwtValidator, AuthenticatedUser};
use models::api::{Contact, OutgoingModel};
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
        .mount("/", routes![refresh_my_contacts])
}

#[get("/my/<id>/contacts")]
async fn refresh_my_contacts(
    id: String,
    user: AuthenticatedUser,
    mongo: State<'_, MongoManager>,
) -> Result<Option<OutgoingModel<Contact>>, Status> {
    if user.id() != &id {
        // Return `None`, i.e. a 404, if the user IDs don't match.
        // Prefer this to a 401, since this way an attacker couldn't
        // use this endpoint to fish for user IDs.
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

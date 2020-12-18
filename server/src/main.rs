#![feature(proc_macro_hygiene, decl_macro)]

use rocket::get;
use rocket_contrib::json::Json;

mod location;
mod openid;
mod user;

use location::{Coordinate, Location};
use openid::{authority::Authority, validator::Validator};
use user::User;

#[tokio::main]
async fn main() {
    let sample_msa_jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IjFMVE16YWtpaGlSbGFfOHoyQkVKVlhlV01xbyJ9.eyJ2ZXIiOiIyLjAiLCJpc3MiOiJodHRwczovL2xvZ2luLm1pY3Jvc29mdG9ubGluZS5jb20vOTE4ODA0MGQtNmM2Ny00YzViLWIxMTItMzZhMzA0YjY2ZGFkL3YyLjAiLCJzdWIiOiJBQUFBQUFBQUFBQUFBQUFBQUFBQUFJUWNnY1RQOVVPS2FXMUo3YjRJOFBjIiwiYXVkIjoiOTdiNTkwMGQtYmRiZS00MWJmLThhZmItMzlmZGNiMDk5M2VlIiwiZXhwIjoxNjA4MzQ1NzI3LCJpYXQiOjE2MDgyNTkwMjcsIm5iZiI6MTYwODI1OTAyNywidGlkIjoiOTE4ODA0MGQtNmM2Ny00YzViLWIxMTItMzZhMzA0YjY2ZGFkIiwibm9uY2UiOiJNTWxiTFh6MmNNR2pvM1VZS1JwZi1tc3B4ckExY0xHWEd5WXZfb1prRFpZIiwiYWlvIjoiRFpNc05EZFp5S014aWNzS2k1NTE0b0EwN0JOdktHIW5vMGtPdGNwZFEyQ3RaVjdsYWhtTTUzVlc3Q3A3cU5QVUVybVNhRkJ1ZjdVdlA0S2ZseGtlQ0E1SXA4elJkc2NSUWM5VGRicUMqcUVuZyFyMm5Qclh6TE5xaGZCSnZwSmJlb3BmQVZXeUdjQjhibXVpZ0N6b1dVYUtsVU1senRyZXliVVF0MlJzUE4wUWZoWDZnUm1VZ2dBeSpZeTg5bkFzNkEkJCJ9.o9PxF3OsVq0YtxU49TNfI74HJJJ275v_OdBDwBZKJSktYiAW5cdgiQrxc05LJnsvtPJATTZh4Z8VemRBJOG0wRfj3I6Q5Eq5hvp8T4YAIAbcatS3kpd5QQs5Up4LbvF0uELn7sFjbdja__HBylUxjyJW_Qq3KuE7u6fuLGINM94-5nvyPocJMu7wb1T_ZIstAAx_QdMlLVKMtK6Hd6P55PU4YIseHvK5Iq3DrFg5Pmap-qxh72uucn8ateFRaTqyh-XBifM_h6-AC_JGTZ3b7j5b87DRrZjuDILKg6CQNTry6A8bxlNwgsD706gwSM2XV8C5h8P-HXuuGYEzl1pZ5w";

    let mut validator = Validator::new(Authority::MSA);

    println!("Is valid: {}", validator.validate(&sample_msa_jwt).await);

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

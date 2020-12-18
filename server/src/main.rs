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
    let sample_msa_jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IjFMVE16YWtpaGlSbGFfOHoyQkVKVlhlV01xbyJ9.eyJ2ZXIiOiIyLjAiLCJpc3MiOiJodHRwczovL2xvZ2luLm1pY3Jvc29mdG9ubGluZS5jb20vOTE4ODA0MGQtNmM2Ny00YzViLWIxMTItMzZhMzA0YjY2ZGFkL3YyLjAiLCJzdWIiOiJBQUFBQUFBQUFBQUFBQUFBQUFBQUFJUWNnY1RQOVVPS2FXMUo3YjRJOFBjIiwiYXVkIjoiOTdiNTkwMGQtYmRiZS00MWJmLThhZmItMzlmZGNiMDk5M2VlIiwiZXhwIjoxNjA4MzQyNTIyLCJpYXQiOjE2MDgyNTU4MjIsIm5iZiI6MTYwODI1NTgyMiwidGlkIjoiOTE4ODA0MGQtNmM2Ny00YzViLWIxMTItMzZhMzA0YjY2ZGFkIiwibm9uY2UiOiJNWXVfTFVmZkQtVGxERkswbEJpOFF3VWMxX2M2dVVzd1NmWVlxWHZaLUhBIiwiYWlvIjoiRGR1SVRuc3hrNDFXN2o5MmdlWjVrVzR0S29wbEtSRjVRSVRJaVRCbkM0dU9udmtGWExoMDNRdHNMNnBuSFhTY0lmNmViYnowVW5WTFJIZ2VacGtQM2dWYm5ONjAyaHJJZWlET08yaGxpUUg5WlRYRTdncnRibE5veTFab1JGczdyc21MbSo3djVuWDhvUjVyV28hdmNvS08qMUJQajNoZlR6SXFESmhrNUVaR2FlUk45eCEwa3dzQTR1clNkWDdtZHckJCJ9.aCDaxYnh7kAFmEezjCe8l4S7rnarA1e_TQ7kjHm_BlFeIZCqedBQk4OKOaHxv3WSm6ftB6Z1ZKgrVZChsQTImoiYk6VE74aAF6deDDybq1z4PAwhX7-FFnGqerAk_NnzMVdh6V8uAkZyktkpwC5U5b82v7sRV23jxqQqJo94OaNcXSTBHbk6tbtVIjmS0sgWZ6vhkBCm2DtQ19pAhPNVHLA_L29YzO9U_JzZbgOqNiFAoSZWTm1HDqk2f5e5915SlJolk7AMEQCei9MX0kMk1N5zVrjF8zyKbGIQ7pkWAD_BdHCPgf6UoJMADLG7-TgBFy0hMeD5GF-68DHgiGCEug";

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

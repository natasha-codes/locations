use rocket_contrib::json::Json;
use serde::Serialize;

use crate::models::api::ApiError;

pub type RouteResult<T> = std::result::Result<Json<T>, ApiError>;

pub trait ToRouteResult<T> {
    fn to_route_result(self) -> RouteResult<T>;
}

impl<T, E: Into<ApiError>> ToRouteResult<T> for std::result::Result<T, E> {
    fn to_route_result(self) -> RouteResult<T> {
        match self {
            Ok(ok_val) => Ok(Json(ok_val)),
            Err(err) => Err(err.into()),
        }
    }
}

impl<T: Serialize> ToRouteResult<T> for T {
    fn to_route_result(self) -> RouteResult<T> {
        Ok(Json(self))
    }
}

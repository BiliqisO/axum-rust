use axum::{http::StatusCode};
use axum::response::{IntoResponse, Response};
use serde_json::error;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error  {
    LoginFail,

    // ---Auth errors
    AUthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    // ---Model errors
    TicketDeleteFailNotFound{id:u64}
}
impl IntoResponse for Error  {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    LoginFail,

    // AUTH errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    // Model errors
    TicketDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        dbg!(self);
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

use axum::{http::StatusCode, response::IntoResponse};

// here i am publically exporting the Reult Type to handle the error of our custom error
pub type Result<T> = std::result::Result<T, Error>;

// giving my own type of error enum
#[derive(Debug)]
pub enum Error {
    LoginFail,

    // Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
    TicketCreateFailedEmptyTitle,
}

// this is must to impl
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        std::println!("->> {:<12} - {self:?}", "INTO_RES");

        // this tuple will be turned to a Response type of the server
        // NOTE: NEVER SEND THE SERVER ERROR CODE TO THE CLIENT AS IT IS A SECURITY RISK
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

// this is the good practise the handle the error and using it to display on the screen during the debug
// this is basically implementing the parent Display Module to handle the error for my particular type -->
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

// similar to the above making the standard crate handle the custom Error type -->
impl std::error::Error for Error {}

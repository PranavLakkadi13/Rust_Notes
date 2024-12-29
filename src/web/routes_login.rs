use crate::{web::AUTH_TOKEN, Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::result::Result::Ok;
use tower_cookies::{Cookie, Cookies};

// the payload of the login: basically the data that will be sent from the client
#[derive(Debug, Deserialize)]
pub struct LoginPayLoad {
    pub username: String,
    pub pwd: String,
}

/// here we are supposed to have a router to handle the routes and return it for the whole module
/// this can be merged with the main router handler in the main.rs file
pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

// the JSON here is called a body extracter and there can be only 1 of it per route and it has to be the
// last argument of the fn
async fn api_login(cookies: Cookies, payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    std::println!("->> {:<32} - api_login", "handler");

    // todo!! implement a real db logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // here in the future we can plan to add cookies that can bve sent to the user and keep session
    // here we are hardcoding the use of cookies and use as an auth token and value is user id with exipry and the sign --> example format
    // FIXME: THIS IS THE PLACE WHERE TO PLACE A REAL AUTHENTICATION TOKEN SIGNATURE/GENERATON
    cookies.add(Cookie::new(AUTH_TOKEN, "user.1.exp.sign"));

    // this is the result to the client
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    // used to return the result body
    Ok(body)
}

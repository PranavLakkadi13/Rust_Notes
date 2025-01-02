use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::ctx::Context;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

use axum::body::Body;

// <B> is for the generic type Body
pub async fn mw_require_auth(
    ctx: Result<Context>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<32} - mw_require_auth ", "MIDDLEWARE");

    // // basically parsing the auth token form that file to this and converting to a Option<String>
    // // basically checks the existance of cookie for auth
    // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // // TODO Real Auth Token Parsing and Validation
    // let (user_id, expiration, sign) = auth_token
    //     .ok_or(Error::AuthFailNoAuthTokenCookie)

    ctx?;

    // TODO -> Validating the returned data and the type

    Ok(next.run(req).await)
}

// Start Region ---> for the custom extractor to get user_id  (CTX Extractor)

// here we are using something called as an async trait basically it allows the trait to internally have an async function
// the generic Send/Sync are restrictive and give any indormation about the body respectively
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        std::println!("->> {:<32} - ctx", "Extractor");

        // using the cookie extractor
        let cookies = parts.extract::<Cookies>().await.unwrap();

        // basically parsing the auth token form that file to this and converting to a Option<String>
        // basically checks the existance of cookie for auth
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // TODO Real Auth Token Parsing and Validation
        let (user_id, expiration, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?; //-> here only checking the right format

        // TODO -> Validating the returned data and the type

        // here we are returning the context data to be used for better auth
        Ok(Context::new(user_id))
    }
}

// End Region ---> for the custom extractor (CTX Extractor)

// Here we will see and learn about token parsing in the format 'user-[user_id].[expiration].[signature]'
// returns tuple -> (user_id , expiration, signature)
fn parse_token(token: String) -> Result<(i64, String, String)> {
    // extracting the data from the string and breaking into slices to extract meaningful data
    // the regex returns the needed values the values in the "()"  after the "\." will be retuned to the respective values
    let (_whole, user_id, expiration, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    // here we will type caste to the values we need of a particular type
    let user_id: i64 = user_id
        .parse()
        .map_err(|e| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, expiration.to_string(), sign.to_string()))
}

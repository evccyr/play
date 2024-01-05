use crate::{web, Error, Result};
use axum::{
    routing::{post, Router},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    dbg!(">>>>>> API_LOGIN HANDLER");

    // todo!("Real Auth Logic")
    if payload.uname != "yash" || payload.pwd != "passwd" {
        return Err(Error::LoginFail);
    }

    // Manage Cookies
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
    "result":{
    "success":true
    }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    uname: String, // username
    pwd: String,   // password
}

use axum::{async_trait, RequestPartsExt};
use axum::middleware::Next;
use axum::http::{ Request, request::Parts };
use axum::response::Response;
use tower_cookies::Cookies;
use axum::body::Body;
use axum::extract::FromRequestParts;
use lazy_regex::regex_captures;
use crate::ctx::Ctx;
use crate::{Result, Error };
use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req:Request<Body>,
    next:Next
) -> Result<Response> {
    dbg!("-----REQUIRE AUTH------");

    ctx?;
    Ok(next.run(req).await)
}

/// Parse auth token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token:String)-> Result<(u64,String,String)> {
    let (_whole,user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id:u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}


#[async_trait]
impl<S:Send+Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts:&mut Parts, _state:&S) -> Result<Self> {
        dbg!("->> CTX EXTRACTOR");

        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Auth token parsing & validation
        let(user_id,exp,sign)= auth_token.ok_or(Error::AuthFailNoAuthTokenCookie).and_then(parse_token)?;

        Ok(Ctx::new(user_id))
    }
}

use async_trait::async_trait;
use axum::http::Request;
use axum::middleware::Next;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
// use axum::RequestPartsExt;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use crate::ctx::Ctx;
use crate::model::ModelController;

pub async fn my_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next,) -> Result<Response> {
    println!("->> {:<12} - my_require_auth - {ctx:?}", "MIDDLEWARE");
    // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    // let (user_id, exp, sign) = auth_token
    //     .ok_or(Error::AuthFailNoAuthTokenCookie)
    //     .and_then(parse_token)?;

    ctx?;

    Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    // Compute Result<Ctx>
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, exp, sign)) => {
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove hte cookies if something went wrong other than NoAuthTokenCookie
    if result_ctx.is_err()
        && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie))
    {
        // cookies.remove(Cookie::named(AUTH_TOKEN));
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) ->Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        // // User the cookies' extractor.
        // let cookies = parts.extract::<Cookies>().await.unwrap();
        // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
        //
        // // Parse token
        // let (user_id, exp, sign) = auth_token
        //     .ok_or(Error::AuthFailNoAuthTokenCookie)
        //     .and_then(parse_token)?;

        // Ok(Ctx::new(user_id))

        parts.extensions.get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

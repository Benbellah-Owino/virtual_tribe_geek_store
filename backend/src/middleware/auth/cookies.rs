use axum::http::StatusCode;
use axum::middleware::Next;
use axum::{extract::Request, response::Response};
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::json;
use std::env;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};
use tower_cookies::{Cookie, Cookies};
use tracing::error;

use crate::middleware::auth::jwt::gen_token;

use super::{
    jwt::{decode_token, Claims},
    AuthError,
};

#[derive(Clone, Debug)]
struct RecordId<'a> {
    table: &'a str,
    id: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserRefreshToken {
    pub id: Thing,
    pub refresh_token: String,
}
// section:      -- error
#[derive(Debug)]
pub enum CookieError {
    MissingCookieError,
}
// endsection:   -- error

/// Used to generate an auth cookie and add it to Cookies
///
/// # Example
/// fn gen_cookie(){
///    use backend::middleware::auth::cookies::gen_auth_cookie;
/// if let Ok(s) = gen_auth_cookie(&claims, &cookies){
///         println!("{}", s);
///     }else{
///         eprintln!("Cookies not added correctly")
///    }
/// }
/// ```
pub fn gen_auth_cookie(claims: &Claims, cookies: &Cookies) -> Result<&'static str, AuthError> {
    dotenv().ok();

    let mut c = claims.to_owned(); //Clone the claims struct
    c.exp += 10_800; //Change the expiry to 3 hours
    let secret = &env::var("AUTH_TOKEN").expect("Set auth_token"); //Get the auth token secret
    let token = gen_token(&c, secret); //Generate the auth token
    if let Ok(s) = token {
        // TODO: Create a cookie with cookie.builder
        let cookie = Cookie::build(("auth_token", s)).path("/");
        cookies.add(Cookie::from(cookie)); //add it to cookies
        Ok("Auth cookie added")
    } else {
        Err(AuthError::AuthTokenError)
    }
}

pub async fn gen_refresh_cookie(
    claims: &Claims,
    db: &Surreal<Client>,
) -> Result<Option<UserRefreshToken>, AuthError> {
    dotenv().ok();

    let mut c = claims.to_owned(); //Clone the claims struct

    let id = &c.id.to_owned().to_string();
    let id: Vec<&str> = id.split(":").collect();

    let id = RecordId {
        table: id[0],
        id: id[1],
    };
    c.exp += 604_800; //Change the expiry to 1 week
    let secret = &env::var("REFRESH_TOKEN").expect("Set refresh_token_secret"); // Generate secret
    let token = gen_token(&c, secret); //Generate the token
    if let Ok(s) = token {
        let query: Option<UserRefreshToken> = db
            .update((id.table, id.id))
            .merge(json!({"refresh_token": s}))
            .await
            .unwrap(); // Update the refresh token
        Ok(query)
    } else {
        Err(AuthError::AuthTokenError)
    }
}

pub async fn verify_user(
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_token = cookies
        .get("auth_token")
        .map(|t| t.to_string())
        .ok_or(CookieError::MissingCookieError);
    match auth_token {
        Ok(t) => {
            let token_split: Vec<&str> = t.split("=").collect();
            let secret = &env::var("AUTH_TOKEN").expect("Set auth_token"); // Generate secret
            match decode_token(token_split[1], secret) {
                Ok(c) => {
                    request.extensions_mut().insert(c.id.clone());
                    Ok(next.run(request).await)
                }
                Err(e) => {
                    error!("{e}");
                    Err(StatusCode::UNAUTHORIZED)
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

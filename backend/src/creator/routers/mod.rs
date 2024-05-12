// section:      -- imports
use axum::{
    middleware, routing::*, Json, Router
};
use axum::response::IntoResponse;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_cookies::Cookies;


use crate::dev_initial::db::Db;
use crate::middleware::auth::cookies::{gen_auth_cookie, gen_refresh_cookie, verify_user};

use super::controllers::{register, login, get_details, update_details};
use super::{Creator, CreatorError, CreatorForCreate, CreatorForLogin, CreatorForUpdate};
// endsection:   -- imports



// section:      -- router
pub fn creator_router() -> Router<Db>{
    return Router::new()
        .route("/", patch(details_update_handler))
        .route("/", get(details_handler))
        .layer(middleware::from_fn(verify_user))
        .route("/", post(register_handler))
        .route("/login", get(login_handler))
}
// endsection:   -- router



// section:      -- handlers
#[axum_macros::debug_handler]
async fn register_handler(State(db): State<Db>, Json(payload): Json<CreatorForCreate>) -> impl IntoResponse{
    dbg!(&payload);
    let db = db.unwrap();
    let creator = register(&db, payload).await;

    match creator{
        Ok(_c) => {
            return (StatusCode::CREATED).into_response() 
        },
        Err(_) => {
            
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}



#[axum_macros::debug_handler]
async fn login_handler(State(db): State<Db>, cookies:Cookies,  Json(payload): Json<CreatorForLogin>) -> impl IntoResponse{
    dbg!(&payload);
    let db = db.unwrap();
    let creator = login(&db, payload).await;

    match creator{
        Ok(claims) => {
            match gen_auth_cookie(&claims, &cookies){  // Generating the auth token and saving it as a cookie then handling the error
                Ok(_) => {
                    if let Ok(refresh_token) = gen_refresh_cookie(&claims, &db).await{ //Generate refresh token and save it to db
                        println!("{:?}", refresh_token);
                    }

                },
                Err(_) =>{
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
            }
            return (StatusCode::ACCEPTED, Json(json!({"creator":claims}))).into_response() 
        },
        Err(e) => {
            match e{
                crate::creator::CreatorError::LoginError => {
                    
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                },
                crate::creator::CreatorError::WrongCredentialsError => {

                    return (StatusCode::UNAUTHORIZED).into_response()
                },
                crate::creator::CreatorError::LoginAttemptsError => {

                    return (StatusCode::FORBIDDEN, Json(json!({"msg":"Too many login attempts"}))).into_response()
                }
                _ =>{
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
            }
        }
    }
}


pub async fn details_handler(State(db): State<Db>,cookies:Cookies, req:Request) -> impl IntoResponse{
    if let Some(id) = req.extensions().get::<String>(){
        println!("{id}");
        let db = db.unwrap();
        let creator = get_details(&db, id.to_owned()).await;
        match creator{
            Ok(c) => {
                return (StatusCode::FOUND, Json(json!({"creator": c})))
            }
            Err(_) => {
                return (StatusCode::NOT_FOUND, Json(json!({"msg": "Creator not found"})))
            }
        }
    }else{
        println!("Error");
        return (StatusCode::NOT_FOUND, Json(json!({"msg": "Creator not found"})))
    }

}

pub async fn details_update_handler<I: Serialize + for<'a> Deserialize<'a>>(State(db): State<Db>, Json(payload):Json<Vec<CreatorForUpdate<I>>>, req:Request) -> impl IntoResponse{
    let mut if_error = false;
    return if let Some(id) = req.extensions().get::<String>() {
        println!("{id}");
        let db = db.unwrap();
        for i in payload{
            let creator = update_details(&db, id.to_owned(), &i.field, i.value).await;

            match creator{
                Ok(_) => {
                    if_error = false;
                }
                Err(_) => {
                    if_error = true;
                }
            }

        }
        match if_error {
            true => {
                let creator = get_details(&db, id.to_owned()).await.unwrap();
                (StatusCode::FOUND, Json(json!({"creator": creator})))
            }
            false => {
                let creator = get_details(&db, id.to_owned()).await.unwrap();
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"msg": "Update error", "c":creator})))
            }
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"msg": "Creator not updated"})))
    }

}
// endsection:   -- handlers



// section:      -- imports
// endsection:   -- imports



// section:      -- imports
// endsection:   -- imports
use axum::{
    body::to_bytes,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};
use tracing::{debug, error, info};

use crate::{dev_initial::db::Db, vrt_lib::surreal_db_fns::check_owner_2};

use super::{
    controllers::{create, delete_studio, get_all, get_details, update},
    Studio, StudioError, StudioForCreate, StudioUpdateClient,
};

struct OwnerStudio {
    creator: Thing,
    studio: Thing,
}
//TODO: Finishs
pub fn studio_router() -> Router<Db> {
    Router::new()
        .route(
            "/:studio_id",
            get(get_one).patch(update_handler).delete(delete_handler),
        )
        .route("/", post(create_handler))
        .layer(middleware::from_fn(
            crate::middleware::auth::cookies::verify_user,
        ))
        .route("/", get(get_all_handler))
}
// section:      -- handlers

/// <h1> Handles registration of studio </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /studio/ </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "name":"test_studio1", <br>
///     "email":"test_studio1@gmail.com", <br>
///     "owner":"creator:"ds8dxstwqut33x" <br>
/// }<br><br>
///
/// <h5>
///     Parameters can be empty
/// </h5>

pub async fn create_handler(State(db): State<Db>, req: Request) -> impl IntoResponse {
    let db = db.unwrap();
    println!("CREATE");

    if let Some(id) = req.extensions().get::<String>() {
        let creator_id = id.clone(); // It's cloned since request is consumed in the next section
        dbg!(&creator_id);
        // the 2 lines below extract request body and serialize it into the correct format
        let body_bytes = to_bytes(req.into_body(), 10480).await.unwrap();
        info!("tag1");
        let mut payload: StudioForCreate = serde_json::from_slice(&body_bytes).unwrap();

        info!("tag2");
        payload.owner = creator_id.clone(); //Setting the owner to be the creator making th POST request. Creator must be logged in to do so

        eprintln!("created {:?}", &payload);
        info!("tag3");
        let studio = create(&db, payload).await; //Saving the studio to database

        info!("tag4");
        match studio {
            //Useful for ensuring the operation was succesfull
            Ok(s) => {
                eprintln!("created {:?}", &s);
                info!("tag5");
                return (StatusCode::CREATED, Json(json!({"payload": s})));
            }
            Err(e) => {
                info!("tag6");
                eprintln!("{:?}", e);
            }
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"msg": "Error creating studio"})),
    )
}

/// <h1> Handles retrieval of studios </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /studio/ </b> </h2>
///
/// <h3> Request body</h3>
/// NO BODY
///
/// <h5>
///     Parameters can be empty
/// </h5>
#[axum_macros::debug_handler]
pub async fn get_all_handler(State(db): State<Db>, req: Request) -> impl IntoResponse {
    // Subject to make free
    let db = db.unwrap();
    println!("GET ALL");
    let studios = get_all(&db).await;

    match studios {
        Ok(s) => (StatusCode::OK, Json(json!({"payload": s}))),
        Err(e) => {
            debug!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg": "Error getting studios"})),
            )
        }
    }
}

/// <h1> Handles retrieval of one studio </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /studio/{id} </b> </h2>
///
/// <h3> Request body</h3>
/// NO BODY
///
/// <h5>
///     Parameters can be empty
/// </h5>
#[axum_macros::debug_handler]
pub async fn get_one(State(db): State<Db>, Path(studio_id): Path<String>) -> impl IntoResponse {
    // Subject to make free
    let db = db.unwrap();
    println!("{studio_id}");

    let studio = get_details(&db, studio_id).await;

    println!("{:?}", studio);
    match studio {
        Ok(s) => (StatusCode::OK, Json(json!({"studio": s}))),
        Err(e) => {
            debug!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg": "Error getting studio"})),
            )
        }
    }
}

pub async fn check_owner(
    creator: &String,
    studio: &String,
    db: &Surreal<Client>,
) -> Result<Studio, StudioError> {
    let query = db
        .query("SELECT * FROM studio WHERE id = $id")
        .bind(("id", studio.clone())) // Clone studio here to avoid temporary borrow
        .await;

    match query {
        Ok(mut res) => {
            dbg!(&res);
            let studio: Result<Vec<Studio>, surrealdb::Error> = res.take(0);
            
            if let Ok(s) = studio {
                if s.is_empty() {
                    return Err(StudioError::StudioRetrievingError);
                }

                // Move the first element to force full ownership
                let owned_studio = s.into_iter().next().unwrap(); 

                let creator_parts: Vec<String> = creator.split(':').map(|s| s.to_string()).collect();
                
                if owned_studio.owner.id.to_string() == creator_parts[1] && owned_studio.owner.tb == creator_parts[0] {
                    Ok(owned_studio)
                } else {
                    Err(StudioError::OwnerMismatch)
                }
            } else {
                Err(StudioError::StudioRetrievingError)
            }
        }
        Err(_) => Err(StudioError::StudioRetrievingError),
    }
}


/// <h1> Handles updating of studio </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /studio/ </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>[<br>
///     "field":"email", <br>
///     "value":"test_studio1@gmail.com", <br>
///     ]<br>
/// }<br><br>
///
/// <h5>
///     Parameters can be empty
/// </h5>
pub async fn update_handler(
    State(db): State<Db>,
    Path(studio_id): Path<String>,
    req: Request,
) -> impl IntoResponse {
    let db = db.unwrap();
    let mut is_error = false; //Checksum
    let studio_id = studio_id.clone();
    println!("{studio_id}");
    if let Some(id) = req.extensions().get::<String>() {
        let creator_id = id.clone(); // It's cloned since request is consumed in the next section

        // the 2 lines below extract request body and serialize it into the correct format
        //TODO: Test for the suitable amount of bytes
        let body_bytes = to_bytes(req.into_body(), 2480).await.unwrap();
        let payload: StudioUpdateClient = serde_json::from_slice(&body_bytes).unwrap();

        eprintln!("{:#?}", payload);
        let len_t = payload.payload.len() as u8 - 1u8; //Get the last index
        match check_owner_2(&creator_id, &studio_id, &db).await {
            //Check if the client is the owner of the studio
            Ok(_c) => {
                let mut counter: u8 = 0;

                for i in payload.payload.into_iter() {
                    //iterate through the fields that need updating
                    if let Ok(studio) = update(&db, &studio_id, i).await {
                        if counter == len_t {
                            return (
                                StatusCode::OK,
                                Json(json!({"payload": studio, "error":is_error})),
                            ); // If it's the last item return response
                        }
                    } else {
                        is_error = true; //set error flag to true if error
                    }

                    counter += 1;
                }
            }
            Err(StudioError::OwnerMismatch) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"msg": "Error updating studio"})),
                );
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"msg": "Error updating studio"})),
                );
            }
        } //Setting the owner to be the creator making th POST request. Creator must be logged in to do so
    }

    (
        StatusCode::CONTINUE,
        Json(json!({"msg": "Error updating studio"})),
    )
}

/// <h1> Handles updating of studio </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /studio/ </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>[<br>
///     "field":"email", <br>
///     "value":"test_studio1@gmail.com", <br>
///     ]<br>
/// }<br><br>
///
/// <h5>
///     Parameters can be empty
/// </h5>

pub async fn delete_handler(
    State(db): State<Db>,
    Path(studio_id): Path<String>,
    req: Request,
) -> impl IntoResponse {
    let db = db.unwrap();

    if let Some(creator_id) = req.extensions().get::<String>() {
        let creator_id = creator_id.clone(); // It's cloned since request is consumed in the next section

        match check_owner_2(&creator_id, &studio_id, &db).await {
            //Check if the client is the owner of the studio
            Ok(_c) => {
                if let Ok(s) = delete_studio(&db, studio_id).await {
                    return (
                        StatusCode::OK,
                        Json(json!({"payload": s, "msg":"Studio deleted"})),
                    ); // If it's the last item return response
                } else {
                    error!("Delete Error");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"msg": "Error updating studio"})),
                    );
                }
            }
            Err(StudioError::OwnerMismatch) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"msg": "Error updating studio"})),
                );
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"msg": "Error updating studio"})),
                );
            }
        } //Setting the owner to be the creator making th POST request. Creator must be logged in to do so
    }

    (
        StatusCode::CONTINUE,
        Json(json!({"msg": "Error updating studio"})),
    )
}

// endsection:   -- handlers

// section:      -- handlers
// endsection:   -- handlers
// section:      -- handlers
// endsection:   -- handlers
// section:      -- handlers
// endsection:   -- handlers

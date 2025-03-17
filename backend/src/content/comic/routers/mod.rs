use crate::dev_initial::db::Db;
use axum::routing::Router;

pub fn comic_router() -> Router<Db>{
    Router::new()
}


// region:      --- Handlers

/// <h1> Handles creation of Comic </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /comic </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "writer": "test_creator1", <br>
///     "studio": "studio:sxshus8430sdd9dde"
///     "description": "Nice comic", <br>
///     "audiences": "FAMILY", <br>
///     "recom_price": "password" <br>
/// }<br><br>
///
/// <p>
///     Parameters cannot be empty
/// </p>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 201</li>
///     <li> <b>Err</b> : 500</li>
/// </ul>

// endregion:   --- Handlers

// region:      --- 
// endregion:   --- 

// region:      --- 
// endregion:   --- 

// region:      --- 
// endregion:   --- 

// region:      --- 
// endregion:   --- 
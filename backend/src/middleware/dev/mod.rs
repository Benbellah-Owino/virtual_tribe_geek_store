use axum::{extract::Request, middleware::Next, response::Response};

pub async fn log_request(req: Request, next: Next) -> Response {
    println!("\n-------------------------------------------------------------");
    // Log the request method and URI
    println!("Request: {} {}", req.method(), req.uri());

    // Optionally log headers
    // for (name, value) in req.headers().iter() {
    //     println!("Header: {}: {:?}", name, value);
    // }

    // Pass the request to the next handler
    let response = next.run(req).await;

    // You can also log response details here if needed
    println!("Response status: {}", response.status());

    response
}

use axum::{
    response::{Html, IntoResponse},
    extract::Json,
    response::Json as JsonResponse,
    routing::{get,post},
    Router,
};
use askama::Template;
use serde::{Deserialize, Serialize};
use serde_json;
use std::net::SocketAddr;
/*
async fn handle_post(Json(payload): Json<InputData>) -> JsonResponse<OutputData> {
    let reply = OutputData {
        message: format!("Hello, {}! You are {} years old.", payload.name, payload.age),
    };
    println!("Post request pass!");
    JsonResponse(reply)
    //println!("Payload response: ",JsonResponse(reply)); 
}
*/
//

#[derive(Deserialize)]
struct InputData {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct OutputData {
    message: String,
}
//Get request function end-point
async fn hello() -> &'static str {
    println!("Target end-point trigger");
    "Hello, world!"
}
//Post Request function 
async fn handle_post(Json(payload): Json<InputData>) -> JsonResponse<OutputData> {
    let reply = OutputData {
        message: format!("Hello, {}! You are {} years old.", payload.name, payload.age),
    };

    //*** Print the reply as a JSON string***
    match serde_json::to_string(&reply) {
        Ok(json_string) => println!("Payload response JSON: {}", json_string),
        Err(e) => eprintln!("Failed to serialize JSON: {}", e),
    }

    JsonResponse(reply)
}
// Define your template using Askama
#[derive(Template)]
#[template(path = "main_extract.html")] // looks for templates/index.html
struct IndexTemplate {
    name: String,
}

// GET endpoint that returns rendered HTML
async fn handle_get_html() -> impl IntoResponse {
    let html = IndexTemplate {
        name: "Axum User".to_string(),
    };

    match html.render() {
        Ok(rendered) => Html(rendered).into_response(),
        Err(err) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {}", err),
        )
            .into_response(),
    }
}
#[tokio::main]
async fn main() {
    let app = Router::new()
           .route("/", get(handle_get_html))
           .route("/greet", post(handle_post))
           .route("/hello", get(hello));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

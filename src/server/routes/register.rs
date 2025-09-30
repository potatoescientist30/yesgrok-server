use std::convert::Infallible;
use hyper::{Body, Response, Request, StatusCode};
use serde_json::json;
use crate::server::state::AppState;

const API_URL: &str = "http://localhost:4000";

#[derive(serde::Deserialize)]
struct RegisterRequest {
    agent_id: String,
    agent_url: String
}

pub async fn handle(req: Request<Body>, app_state: AppState) -> Result<Response<Body>, Infallible> {
    let request_body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => return Ok(error_response(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read body: {}", err))),
    };

    let parsed_body: RegisterRequest = match serde_json::from_slice(&request_body) {
        Ok(var) => var,
        Err(_) => {
            return Ok(error_response(StatusCode::BAD_REQUEST, "Invalid body".to_string()))
        }
    };

    let wildcard_url = format!("{}/{}", API_URL, parsed_body.agent_id);
    app_state.routes.write().await.insert(wildcard_url.clone(), parsed_body.agent_url);

    let response_body = json!({ "endpoint": wildcard_url });
    let response_body_parsed = serde_json::to_string(&response_body).unwrap();
    Ok(Response::new(Body::from(response_body_parsed)))
}

fn error_response(status: StatusCode, message: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::from(message))
        .unwrap_or_else(|_| {
            // Fallback response in the very unlikely event builder fails
            Response::new(Body::from("Internal server error"))
        })
}

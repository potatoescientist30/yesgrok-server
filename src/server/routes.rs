use hyper::{Request, Response, Body};
use std::convert::Infallible;
use crate::server::state::AppState;

mod health;
mod register;

pub async fn handle(req: Request<Body>, app_state: AppState) -> Result<Response<Body>, Infallible> {
    match req.uri().path() {
        "/health" => health::handle().await,
        "/register" => register::handle(req, app_state).await,
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}

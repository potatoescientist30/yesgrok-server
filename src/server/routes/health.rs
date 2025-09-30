use hyper::{Body, Response};
use std::convert::Infallible;

pub async fn handle() -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("OK")))
}

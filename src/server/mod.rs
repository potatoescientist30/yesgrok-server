use hyper::{Server, service::{make_service_fn, service_fn}};
use std::{convert::Infallible, net::SocketAddr};

mod routes;
mod state;

pub use routes::handle;
use crate::server::state::AppState;

// Creates a future with captured variables moved in.
// We cannot borrow state from the stack by reference since it will be gone
// at some point by a run of make_service. By using "move", the closure takes ownership
// of the data, which in case of the state it means incrementing the reference counter.
// This way, each connection holds its own clone of the state.

pub async fn run(addr: SocketAddr) {
    let app_state = AppState::new();

    let make_svc = make_service_fn(move |_conn| {
        let state = app_state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let state = state.clone();
                handle(req, state)
            }))
        }
    });

    Server::bind(&addr).serve(make_svc).await.unwrap();
}

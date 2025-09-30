mod server;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    println!("Listening on http://{}", addr);

    server::run(addr).await;
}

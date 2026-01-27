use axum::Router;
use std::net::SocketAddr;
use tracing::info;

pub async fn serve(router: Router, addr: SocketAddr) {
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

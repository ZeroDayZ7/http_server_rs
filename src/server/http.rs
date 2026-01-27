// Copyright 2026 ZeroDayZ7
use axum::Router;
use std::net::SocketAddr;
use tracing::info;

pub async fn serve(router: Router, addr: SocketAddr) {
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    axum::serve(
        listener, 
        router.into_make_service_with_connect_info::<SocketAddr>()
    )
    .await
    .unwrap();
}
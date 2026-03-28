// Copyright 2026 ZeroDayZ7
use axum::Router;
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

pub async fn serve(router: Router, addr: SocketAddr, shutdown_timeout: u64) {
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(shutdown_timeout))
    .await
    .unwrap();
}

async fn shutdown_signal(timeout: u64) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { info!("Received Ctrl+C, starting shutdown..."); },
        _ = terminate => { info!("Received SIGTERM, starting shutdown..."); },
    }

    info!(
        "Waiting maximum {} seconds for active requests to finish...",
        timeout
    );
}

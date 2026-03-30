use http_server_rs::server::state::AppState;
use http_server_rs::{config, server};

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!(error = %e, "❌ Fatal application error");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    // -------------------------
    // 1. CONFIG
    // -------------------------
    let settings = config::load().context("Failed to load configuration")?;
    let settings = Arc::new(settings);

    // -------------------------
    // 2. LOGGING
    // -------------------------

    let _guards = server::logger::init_logging(settings.log.level);

    info!("⚙️ Configuration loaded");

    // -------------------------
    // 3. BUILD STATE (fail-fast)
    // -------------------------
    let state = match AppState::new(settings.clone()).await {
        Ok(s) => s,
        Err(e) => {
            error!(error = ?e, "❌ Krytyczny błąd inicjalizacji AppState");
            std::process::exit(1);
        }
    };

    info!("🧠 Application state initialized");

    // -------------------------
    // 4. ADDRESS
    // -------------------------
    let addr: SocketAddr = format!("{}:{}", settings.server.host, settings.server.port)
        .parse()
        .context("Invalid server address")?;

    // -------------------------
    // 5. ROUTER
    // -------------------------
    let app = server::router(state);

    info!("🚀 Server starting on {}", addr);

    // -------------------------
    // 6. SERVER LIFECYCLE
    // -------------------------
    server::http::serve(app, addr, settings.server.shutdown_timeout)
        .await
        .context("HTTP server crashed")?;

    info!("✅ Server shutdown complete");

    Ok(())
}

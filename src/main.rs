use std::net::SocketAddr;
use http_server_rs::{config, server};

#[tokio::main]
async fn main() {
    let settings = config::load()
        .expect("Failed to load configuration");


    server::init_logging(&settings.log.level);

    tracing::info!("Starting application on {}:{}", 
        settings.server.host, 
        settings.server.port
    );

    let app = server::router();


    let addr = SocketAddr::from((
        settings.server.host.parse::<std::net::IpAddr>()
            .expect("Invalid host IP"),
        settings.server.port,
    ));

    server::http::serve(app, addr).await;
}

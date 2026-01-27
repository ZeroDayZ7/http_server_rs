pub mod http;
pub mod routes;

// funkcja globalna do inicjalizacji logów
use tracing_subscriber::EnvFilter;

pub fn init_logging(level: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(level))
        .init();
}

// reexport routera z routes.rs
pub use routes::router;

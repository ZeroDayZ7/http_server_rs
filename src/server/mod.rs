// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

pub mod http;
pub mod routes;

use tracing_subscriber::EnvFilter;

pub fn init_logging(level: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(level))
        .init();
}


pub use routes::router;

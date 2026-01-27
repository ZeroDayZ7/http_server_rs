// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;
use tracing_appender::rolling::RollingFileAppender;

/// Initializes logging for the application.
/// Logs go both to console and to daily rolling log file.
/// Level is controlled via argument (e.g., "debug" or "info") or LOG__LEVEL env.
pub fn init_logging(level: &str) {
    // Console layer
    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .with_target(false);

    // File layer - daily rolling
    let file_appender: RollingFileAppender = tracing_appender::rolling::daily("logs", "app.log");
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_appender.with_max_level(tracing::Level::DEBUG))
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .with_target(false);

    // EnvFilter for dynamic log level
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    // Combine layers into subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();
}

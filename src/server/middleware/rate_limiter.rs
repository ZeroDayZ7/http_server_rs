use axum::body::Body;
use std::sync::Arc;
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};

use governor::middleware::StateInformationMiddleware;
type AxumGovernorLayer = GovernorLayer<SmartIpKeyExtractor, StateInformationMiddleware, Body>;

#[derive(Clone)]
pub struct RateLimitLayers {
    pub global: AxumGovernorLayer,
    pub health: AxumGovernorLayer,
    pub auth: AxumGovernorLayer,
}

impl RateLimitLayers {
    pub fn new() -> Self {
        let global_conf = GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(3)
            .burst_size(5)
            .use_headers()
            .finish()
            .unwrap();

        let health_conf = GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(360)
            .burst_size(3)
            .use_headers()
            .finish()
            .unwrap();

        let auth_conf = GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(12)
            .burst_size(2)
            .use_headers()
            .finish()
            .unwrap();

        Self {
            global: GovernorLayer::new(Arc::new(global_conf)),
            health: GovernorLayer::new(Arc::new(health_conf)),
            auth: GovernorLayer::new(Arc::new(auth_conf)),
        }
    }
}

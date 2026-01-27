use config::{Config, Environment};
use dotenvy::dotenv;

pub use settings::Settings;

mod settings;

pub fn load() -> Result<Settings, config::ConfigError> {
    dotenv().ok();

    Config::builder()
        .add_source(Environment::default().separator("__"))
        .build()?
        .try_deserialize()
}

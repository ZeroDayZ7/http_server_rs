use crate::config::Settings;
use mongodb::{Client, options::ClientOptions};

pub async fn init_mongo(settings: &Settings) -> mongodb::Database {
    let client_uri = format!(
        "mongodb://{}:{}@{}:{}/{}",
        settings.database.user.as_deref().unwrap_or("admin"),
        settings.database.password.as_deref().unwrap_or("password"),
        settings.database.host,
        settings.database.port,
        settings.database.name
    );

    let mut client_options = ClientOptions::parse(client_uri)
        .await
        .expect("❌ Błędny format URI dla MongoDB");

    client_options.app_name = Some("http_server_rs".to_string());

    let client =
        Client::with_options(client_options).expect("❌ Nie udało się utworzyć klienta MongoDB");

    tracing::info!("🍃 Połączono z MongoDB: {}", settings.database.name);

    // Zwracamy konkretną bazę danych
    client.database(&settings.database.name)
}

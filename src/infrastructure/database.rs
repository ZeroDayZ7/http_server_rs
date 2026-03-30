use crate::config::Settings;
use crate::errors::{AppError, AppResult};
use mongodb::{
    Client, Database,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};
use std::time::Duration;

pub async fn init_mongo(settings: &Settings) -> AppResult<Database> {
    let db_set = &settings.database;

    let auth_source = db_set.auth_source.as_deref().unwrap_or("admin");

    let auth = match (&db_set.user, &db_set.password) {
        (Some(u), Some(p)) if !u.is_empty() && !p.is_empty() => {
            format!("{}:{}@", u, p)
        }
        _ => {
            tracing::warn!(
                "⚠️ Brak poświadczeń DB w konfiguracji - próba połączenia bez autoryzacji"
            );
            String::new()
        }
    };

    let client_uri = format!(
        "mongodb://{}{}:{}/{}?authSource={}&directConnection=true",
        auth, db_set.host, db_set.port, db_set.name, auth_source
    );

    let mut client_options = ClientOptions::parse(&client_uri)
        .await
        .map_err(|e| AppError::ValidationError(format!("Błędny format URI MongoDB: {}", e)))?;

    // --- Senior Refactor: Konfiguracja Puli Połączeń ---
    client_options.app_name = Some("http_server_rs".to_string());
    client_options.max_pool_size = Some(db_set.pool_size);
    client_options.min_pool_size = Some(1); // Utrzymuj przynajmniej jedno połączenie

    // Timeouts
    client_options.connect_timeout = Some(Duration::from_secs(5));
    client_options.server_selection_timeout = Some(Duration::from_secs(5));

    // Stable API
    client_options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());

    let client = Client::with_options(client_options).map_err(|e| {
        AppError::Internal(anyhow::anyhow!(
            "Nie udało się utworzyć klienta MongoDB: {}",
            e
        ))
    })?;

    client
        .database(&db_set.name)
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await
        .map_err(|e| {
            AppError::Internal(anyhow::anyhow!(
                "Brak odpowiedzi od bazy danych '{}' (Ping failed): {}",
                db_set.name,
                e
            ))
        })?;

    tracing::info!(
        "🍃 Pomyślnie zainicjalizowano połączenie z MongoDB: {} (Pool size: {})",
        db_set.name,
        db_set.pool_size
    );

    Ok(client.database(&db_set.name))
}

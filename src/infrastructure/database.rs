// src/infrastructure/database.rs
use crate::config::DatabaseConfig;
use crate::errors::{AppError, AppResult};
use mongodb::bson::doc;
use mongodb::{
    Client, Database,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};

pub async fn init_mongo(db_set: &DatabaseConfig) -> AppResult<Database> {
    let auth_source = db_set.auth_source.as_deref().unwrap_or("admin");

    let auth = match (db_set.user.as_deref(), db_set.password.as_deref()) {
        (Some(u), Some(p)) if !u.is_empty() && !p.is_empty() => {
            format!("{u}:{p}@")
        }
        _ => {
            tracing::warn!("⚠️ Brak poświadczeń DB - próba bez autoryzacji");
            String::new()
        }
    };

    let client_uri = format!(
        "mongodb://{}{}:{}/{}?authSource={}&directConnection=true",
        auth, db_set.host, db_set.port, db_set.name, auth_source
    );

    let mut client_options = ClientOptions::parse(&client_uri)
        .await
        .map_err(|e| AppError::ValidationError(format!("Błędny format URI: {}", e)))?;

    // RETRY LOGIC: Automatyczne ponawianie operacji przy chwilowych problemach z siecią
    client_options.retry_writes = Some(true);
    client_options.retry_reads = Some(true);
    client_options.max_pool_size = Some(db_set.pool_size);
    client_options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());

    let client =
        Client::with_options(client_options).map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    // FAIL-FAST: Sprawdzamy połączenie fizyczne z bazą przed startem serwera.
    // Usunięto 'None', ponieważ run_command w nowym API przyjmuje tylko 1 argument (dokument).
    client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await
        .map_err(|e| AppError::ConfigError(format!("Nie można połączyć z MongoDB: {}", e)))?;

    tracing::info!("✅ Połączono z MongoDB");
    Ok(client.database(&db_set.name))
}

use crate::config::DatabaseConfig;
use crate::errors::{AppError, AppResult};
use mongodb::{
    Client, Database,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};

pub async fn init_mongo(db_set: &DatabaseConfig) -> AppResult<Database> {
    let auth_source = db_set.auth_source.as_deref().unwrap_or("admin");

    // .as_deref() zamienia Option<String> na Option<&str>
    // To najbezpieczniejszy sposób na uniknięcie błędów "cannot infer type"
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

    client_options.max_pool_size = Some(db_set.pool_size);
    client_options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());

    let client =
        Client::with_options(client_options).map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    Ok(client.database(&db_set.name))
}

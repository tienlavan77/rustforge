
use sqlx::{migrate::MigrateError, MySqlPool};

/// Thực thi các database migration chưa được áp dụng.
/// Run all pending database migrations.
pub async fn run_migrations(pool: &MySqlPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    tracing::info!("Database migrations completed");

    Ok(())
}

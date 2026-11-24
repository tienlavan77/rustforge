
use std::time::Duration;

use sqlx::{
    mysql::MySqlPoolOptions,
    MySqlPool,
};

use crate::config::DatabaseSettings;

/// Tạo connection pool kết nối MariaDB.
/// Create a connection pool for MariaDB.
pub async fn create_pool(
    settings: &DatabaseSettings,
) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(settings.max_connections)
        .acquire_timeout(Duration::from_secs(
            settings.acquire_timeout_secs,
        ))
        .connect(&settings.url)
        .await
}

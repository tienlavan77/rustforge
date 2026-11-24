
use sqlx::MySqlPool;

/// Kiểm tra kết nối MariaDB bằng truy vấn đơn giản.
/// Check the MariaDB connection using a simple query.
pub async fn check_database(
    pool: &MySqlPool,
) -> Result<(), sqlx::Error> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await?;

    Ok(())
}

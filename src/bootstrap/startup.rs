
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::{
    app_state::AppState,
    config::Settings,
    infrastructure::database::{
        create_pool,
        health::check_database,
        migration::run_migrations,
    },
    server::router::create_router,
};

/// Khởi tạo cấu hình, database, logging và HTTP server.
/// Initialize configuration, database, logging, and HTTP server.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Đọc và kiểm tra cấu hình ứng dụng.
    // Load and validate application settings.
    let settings = Settings::from_env()?;

    // Khởi tạo logging theo LOG_LEVEL.
    // Initialize logging according to LOG_LEVEL.
    let filter = EnvFilter::try_new(&settings.log_level)?;

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    tracing::info!(
        environment = %settings.environment,
        "ForgeRust configuration loaded"
    );

    // Tạo connection pool cho MariaDB.
    // Create the MariaDB connection pool.
    let database_pool = create_pool(&settings.database).await?;

    run_migrations(&database_pool).await?;

    // Kiểm tra database trước khi khởi động HTTP server.
    // Check the database before starting the HTTP server.
    check_database(&database_pool).await?;

    tracing::info!(
        max_connections = settings.database.max_connections,
        "MariaDB connection established"
    );

    // Khởi tạo trạng thái dùng chung của ứng dụng.
    // Initialize shared application state.
    let app_state = AppState::new(database_pool.clone());

    // Tạo HTTP router.
    // Create the HTTP router.
    let app = create_router(app_state);

    // Khởi tạo TCP listener.
    // Initialize the TCP listener.
    let address = format!("{}:{}", settings.host, settings.port);

    let listener = TcpListener::bind(&address).await?;

    tracing::info!(
        address = %address,
        "ForgeRust server started"
    );

    // Chạy HTTP server và kích hoạt graceful shutdown khi nhận tín hiệu.
    // Run the HTTP server and trigger graceful shutdown when a signal is received.
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("HTTP server stopped");

    // Đóng MariaDB connection pool sau khi HTTP server đã dừng.
    // Close the MariaDB connection pool after the HTTP server has stopped.
    database_pool.close().await;

    tracing::info!("MariaDB connection pool closed");

    tracing::info!("ForgeRust shutdown completed");

    Ok(())
}

/// Chờ tín hiệu Ctrl+C hoặc SIGTERM để bắt đầu graceful shutdown.
/// Wait for Ctrl+C or SIGTERM to initiate graceful shutdown.
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        let mut terminate = signal(SignalKind::terminate())
            .expect("Failed to install SIGTERM handler");

        tokio::select! {
            result = tokio::signal::ctrl_c() => {
                if let Err(error) = result {
                    tracing::error!(
                        error = %error,
                        "Failed to listen for Ctrl+C"
                    );
                }
            }
            _ = terminate.recv() => {
                tracing::info!("Received SIGTERM signal");
            }
        }
    }

    #[cfg(not(unix))]
    {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(
                error = %error,
                "Failed to listen for Ctrl+C"
            );
        }
    }

    tracing::info!("Graceful shutdown signal received");
}

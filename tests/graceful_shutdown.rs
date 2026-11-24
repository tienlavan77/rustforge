
use std::{
    sync::Arc,
    time::Duration,
};

use axum::{
    extract::State,
    routing::get,
    Router,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{oneshot, Notify},
    time::timeout,
};

/// Trạng thái dùng để điều khiển request trong integration test.
/// State used to control the request during the integration test.
struct SlowRequestState {
    /// Thông báo khi handler bắt đầu xử lý request.
    /// Notification sent when the handler starts processing the request.
    started: Notify,

    /// Thông báo cho phép handler hoàn tất xử lý.
    /// Notification allowing the handler to finish processing.
    release: Notify,
}

/// Handler mô phỏng một request xử lý trong thời gian dài.
/// Handler simulating a long-running request.
async fn slow_handler(
    State(state): State<Arc<SlowRequestState>>,
) -> &'static str {
    state.started.notify_one();

    state.release.notified().await;

    "request completed"
}

/// Kiểm tra server chờ request đang xử lý hoàn tất trước khi shutdown.
/// Verify the server waits for an in-flight request before shutting down.
#[tokio::test]
async fn graceful_shutdown_waits_for_in_flight_request() {
    // Tạo trạng thái điều khiển handler.
    // Create handler control state.
    let state = Arc::new(SlowRequestState {
        started: Notify::new(),
        release: Notify::new(),
    });

    // Tạo router phục vụ endpoint kiểm thử.
    // Create a router serving the test endpoint.
    let app = Router::new()
        .route("/slow", get(slow_handler))
        .with_state(state.clone());

    // Khởi tạo TCP listener trên cổng ngẫu nhiên.
    // Initialize a TCP listener on a random port.
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind test listener");

    let address = listener
        .local_addr()
        .expect("Failed to get listener address");

    // Tạo channel điều khiển graceful shutdown.
    // Create a channel to control graceful shutdown.
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // Khởi chạy HTTP server trong background task.
    // Start the HTTP server in a background task.
    let mut server_task = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            })
            .await
    });

    // Kết nối tới HTTP server.
    // Connect to the HTTP server.
    let mut client = TcpStream::connect(address)
        .await
        .expect("Failed to connect to test server");

    // Gửi HTTP request đến endpoint xử lý chậm.
    // Send an HTTP request to the slow endpoint.
    client
        .write_all(
            b"GET /slow HTTP/1.1\r\n\
              Host: localhost\r\n\
              Connection: close\r\n\r\n",
        )
        .await
        .expect("Failed to send HTTP request");

    // Chờ xác nhận handler đã bắt đầu xử lý request.
    // Wait until the handler confirms request processing has started.
    timeout(
        Duration::from_secs(5),
        state.started.notified(),
    )
    .await
    .expect("Handler did not start processing the request");

    // Gửi tín hiệu graceful shutdown.
    // Send the graceful shutdown signal.
    shutdown_tx
        .send(())
        .expect("Failed to send shutdown signal");

    // Xác nhận server chưa kết thúc khi request vẫn đang xử lý.
    // Verify the server has not stopped while the request is still running.
    let shutdown_result = timeout(
        Duration::from_millis(200),
        &mut server_task,
    )
    .await;

    assert!(
        shutdown_result.is_err(),
        "Server stopped before the in-flight request completed"
    );

    // Cho phép handler hoàn tất request.
    // Allow the handler to complete the request.
    state.release.notify_one();

    // Đọc HTTP response từ server.
    // Read the HTTP response from the server.
    let mut response_bytes = Vec::new();

    timeout(
        Duration::from_secs(5),
        client.read_to_end(&mut response_bytes),
    )
    .await
    .expect("Timed out while waiting for HTTP response")
    .expect("Failed to read HTTP response");

    let response = String::from_utf8_lossy(&response_bytes);

    // Xác nhận client nhận được HTTP 200 và response body.
    // Verify the client receives HTTP 200 and the response body.
    assert!(
        response.contains("200 OK"),
        "Expected HTTP 200 response, received: {response}"
    );

    assert!(
        response.contains("request completed"),
        "Expected completed response body, received: {response}"
    );

    // Chờ HTTP server shutdown hoàn tất.
    // Wait for the HTTP server to finish shutting down.
    timeout(
        Duration::from_secs(5),
        &mut server_task,
    )
    .await
    .expect("Server graceful shutdown timed out")
    .expect("Server task panicked")
    .expect("HTTP server returned an error");

    // Xác nhận HTTP server đã shutdown hoàn tất.
   // Confirm the HTTP server has completed shutdown.
   println!("HTTP server graceful shutdown completed");
}

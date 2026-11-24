use axum::{
    body::{to_bytes, Body},
    http::{
        header::CONTENT_TYPE,
        HeaderName,
        HeaderValue,
        Request,
    },
    middleware::Next,
    response::Response,
};
use serde_json::Value;
use uuid::Uuid;

/// Dữ liệu Request ID được lưu trong request extensions.
/// Request ID data stored in request extensions.
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

/// Middleware tạo Request ID và bổ sung vào HTTP response.
/// Middleware generates a Request ID and adds it to the HTTP response.
pub async fn request_id_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Response {
    // Tạo UUID cho mỗi request.
    // Generate a UUID for every request.
    let request_id = Uuid::new_v4().to_string();

    // Lưu Request ID vào request extensions.
    // Store Request ID in request extensions.
    request
        .extensions_mut()
        .insert(RequestId(request_id.clone()));

    // Chuyển request đến handler tiếp theo.
    // Pass the request to the next handler.
    let response = next.run(request).await;

    // Bổ sung Request ID vào response header.
    // Add Request ID to the response header.
    let mut response = response;

    response.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&request_id)
            .unwrap_or_else(|_| HeaderValue::from_static("unknown")),
    );

    // Kiểm tra response có phải JSON lỗi hay không.
    // Check whether the response is a JSON error.
    let is_error = response.status().is_client_error()
        || response.status().is_server_error();

    let is_json = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains("application/json"));

    if !is_error || !is_json {
        return response;
    }

    // Tách response thành header và body.
    // Split the response into headers and body.
    let (parts, body) = response.into_parts();

    // Đọc nội dung body.
    // Read the response body.
    let body_bytes = match to_bytes(body, 1024 * 1024).await {
        Ok(bytes) => bytes,
        Err(error) => {
            tracing::error!(%error, "Failed to read error response body");
            return Response::from_parts(parts, Body::empty());
        }
    };

    // Bổ sung Request ID vào JSON response.
    // Inject Request ID into the JSON response.
    let updated_body = match serde_json::from_slice::<Value>(&body_bytes) {
        Ok(mut value) => {
            if let Some(object) = value.as_object_mut() {
                object.insert(
                    "request_id".to_string(),
                    Value::String(request_id),
                );

                match serde_json::to_vec(&value) {
                    Ok(bytes) => Body::from(bytes),
                    Err(error) => {
                        tracing::error!(%error, "Failed to serialize error response");
                        Body::from(body_bytes)
                    }
                }
            } else {
                Body::from(body_bytes)
            }
        }
        Err(error) => {
            tracing::error!(%error, "Failed to parse error response JSON");
            Body::from(body_bytes)
        }
    };

    Response::from_parts(parts, updated_body)
}
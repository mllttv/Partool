// Dummy implementation for static file server

#[cfg(feature = "ssr")]
use axum::{
    extract::ws::WebSocketUpgrade,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};

#[cfg(feature = "ssr")]
pub fn publish(_service: &str, _topic: &str, _kind: &str) {
    // No-op for static file server
}

#[cfg(feature = "ssr")]
pub async fn handler(_headers: HeaderMap, _ws: WebSocketUpgrade) -> Response {
    (StatusCode::NOT_FOUND, "WebSocket service disabled").into_response()
}

pub fn listen(_service: &'static str, _callback: impl Fn(&str, &str) + 'static) {
    // No-op for static file server
}

include!(concat!(env!("OUT_DIR"), "/dynamic_routes.rs"));

use axum::{
    Router,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use std::{net::SocketAddr, path::Path};
use tokio::fs;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

async fn root_index_handler() -> Response {
    let target_file = ["html/main.html", "html/index.html"]
        .into_iter()
        .find(|p| Path::new(p).exists());

    let Some(path) = target_file else {
        return (
            StatusCode::NOT_FOUND,
            "Neither main.html nor index.html found",
        )
            .into_response();
    };

    match fs::read(path).await {
        Ok(contents) => (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            )],
            contents,
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read html file: {}", err),
        )
            .into_response(),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "partool=debug,tower_http=debug".into()),
        )
        .init();

    if !Path::new("html").exists() && Path::new("../html").exists() {
        let _ = std::env::set_current_dir("..");
    }

    let args: Vec<String> = std::env::args().collect();
    let get_arg = |flag| args.windows(2).find(|w| w[0] == flag).map(|w| &w[1][..]);
    let host = get_arg("--host").unwrap_or("127.0.0.1");
    let port = get_arg("--port").unwrap_or("3000");
    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("Invalid --host or --port");

    let app = Router::new()
        .route("/", get(root_index_handler))
        .merge(build_dynamic_router())
        .fallback_service(ServeDir::new("html"))
        .layer(TraceLayer::new_for_http());

    println!("🚀 Partool Server starting on http://{}", addr);
    println!("📄 Index route:   https://domain/ -> html/main.html (fallback: html/index.html)");
    println!("📦 Dynamic routes: https://domain/path/to/file.rs -> ./src/routes/path/to/file.rs");
    println!("📁 Static routes:  https://domain/path/to/file.ext -> html/path/to/file.ext");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

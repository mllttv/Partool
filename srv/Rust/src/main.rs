#![recursion_limit = "256"]

include!(concat!(env!("OUT_DIR"), "/dynamic_routes.rs"));

pub mod app;
pub mod components;

use axum::{
    Router,
    extract::Request,
    http::{HeaderName, HeaderValue, header},
    middleware::map_response,
    response::{Html, IntoResponse, Response},
    routing::{any, get},
};
use std::{env, net::SocketAddr, path::Path};
use tokio::{fs, net::TcpListener};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use components::{keys, status, ws};

async fn root_index_handler(req: Request) -> Response {
    status::increment_access();
    if let Ok(body) = fs::read("html/index.html").await {
        Html(body).into_response()
    } else {
        (leptos_axum::render_app_to_stream(app::Shell))(req).await.into_response()
    }
}

async fn add_security_headers(mut res: Response) -> Response {
    let h = res.headers_mut();
    for (k, v) in [
        (header::X_CONTENT_TYPE_OPTIONS.as_str(), "nosniff"),
        (header::X_FRAME_OPTIONS.as_str(), "DENY"),
        (header::STRICT_TRANSPORT_SECURITY.as_str(), "max-age=31536000; includeSubDomains"),
        (header::REFERRER_POLICY.as_str(), "strict-origin-when-cross-origin"),
        (
            header::CONTENT_SECURITY_POLICY.as_str(),
            "default-src 'self'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; connect-src 'self' ws: wss:; img-src 'self' data: blob:;",
        ),
        ("permissions-policy", "camera=(), microphone=(), geolocation=()"),
    ] {
        h.insert(HeaderName::from_static(k), HeaderValue::from_static(v));
    }
    res
}

#[tokio::main]
async fn main() {
    let _ = any_spawner::Executor::init_tokio();

    tracing_subscriber::fmt().with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "htmlvm=debug,tower_http=debug".into())).init();

    if !Path::new("html").exists() {
        let _ = ["..", "srv"].into_iter().find(|d| Path::new(d).join("html").exists()).map(env::set_current_dir);
    }

    keys::init();
    let _ = status::init();

    let cli_args: Vec<String> = env::args().collect();
    let get_arg = |flag| cli_args.windows(2).find(|w| w[0] == flag).map(|w| &w[1][..]);
    let addr: SocketAddr = format!("{}:{}", get_arg("--host").unwrap_or("127.0.0.1"), get_arg("--port").unwrap_or("3000")).parse().expect("Invalid --host or --port");

    let app = Router::new()
        .route("/", get(root_index_handler))
        .route("/ws", get(ws::handler))
        .route("/api/{*fn_name}", any(leptos_axum::handle_server_fns))
        .fallback_service(ServeDir::new("html").fallback(get(leptos_axum::render_app_to_stream(app::Shell))))
        .layer(map_response(add_security_headers))
        .layer(TraceLayer::new_for_http());

    println!("🚀 Static File Server (HTMLVM) starting on http://{addr}");
    println!("📄 Index route:   https://domain/ -> html/index.html (fallback: /routes/index.rs)");
    println!("📁 Static routes:  https://domain/path/to/file.ext -> html/path/to/file.ext");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

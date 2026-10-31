// Dummy implementation for static file server

use axum::http::HeaderMap;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Claims {
    pub bekdor: bool,
    pub iat: usize,
}

pub fn auth_cookie(_headers: &HeaderMap) -> Result<(), ServerFnError> {
    Err(ServerFnError::new("Bekdor is disabled for static file server"))
}

pub fn issue_challenge() -> String {
    String::new()
}

#[server(prefix = "/bekdor", endpoint = "login", input = server_fn::codec::Json)]
pub async fn login_action(_challenge: String, _signature: String) -> Result<(), ServerFnError> {
    Err(ServerFnError::new("Bekdor disabled"))
}

#[server(prefix = "/bekdor", endpoint = "logout", input = server_fn::codec::Json)]
pub async fn logout_action() -> Result<(), ServerFnError> {
    Ok(())
}

#[server(prefix = "/bekdor", endpoint = "update_jwt", input = server_fn::codec::Json)]
pub async fn update_jwt_action() -> Result<(), ServerFnError> {
    Err(ServerFnError::new("Bekdor disabled"))
}

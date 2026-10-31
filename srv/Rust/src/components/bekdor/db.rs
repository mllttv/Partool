// Dummy implementation for static file server

use leptos::prelude::*;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountRecord {
    pub id: i64,
    pub username: String,
    pub mail: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: i64,
    pub created_at: u64,
}

pub fn get_db() -> Result<Connection, ServerFnError> {
    Connection::open_in_memory().map_err(|e| ServerFnError::new(e.to_string()))
}

pub fn get_var_7z_path() -> PathBuf {
    PathBuf::new()
}

pub fn query_accounts(_q: &str) -> Result<Vec<AccountRecord>, ServerFnError> {
    Ok(Vec::new())
}

pub fn archive_card_view(_exists: bool, _size: u64) -> impl IntoView {
    view! { <div/> }
}

pub fn status_card_view(_today: i64, _total: i64, _version: &str) -> impl IntoView {
    view! { <div/> }
}

#[server(prefix = "/bekdor", endpoint = "delete_account", input = server_fn::codec::Json)]
pub async fn delete_account_action(_id: i64) -> Result<(), ServerFnError> {
    Err(ServerFnError::new("Bekdor disabled"))
}

#[server(prefix = "/bekdor", endpoint = "update_account", input = server_fn::codec::Json)]
pub async fn update_account_action(_id: i64, _username: String, _mail: Option<String>, _role: i64) -> Result<(), ServerFnError> {
    Err(ServerFnError::new("Bekdor disabled"))
}

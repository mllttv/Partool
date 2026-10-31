// Dummy implementation for static file server

use rusqlite::Connection;
use server_fn::{
    ServerFnError,
    codec::{ByteStream, MultipartData},
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct Entry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub time: u64,
}

pub fn dir() -> PathBuf {
    ["/srv/var", "srv/var", "../var", "var"]
        .into_iter()
        .find(|p| Path::new(p).exists())
        .map(PathBuf::from)
        .unwrap_or_else(|| "var".into())
}

pub async fn write(_service: &str, _rel: &str, _data: MultipartData) -> Result<(HashMap<String, String>, Option<(String, u64)>), ServerFnError> {
    Err(ServerFnError::new("Storage write disabled for static file server"))
}

pub async fn read(_service: &str, _rel: &str, _filename: &str) -> Result<ByteStream, ServerFnError> {
    Err(ServerFnError::new("Storage read disabled for static file server"))
}

pub fn mv(_service: &str, _from: &str, _to: &str) -> Result<(), ServerFnError> {
    Ok(())
}

pub fn remove(_service: &str, _rel: &str) -> Result<(), ServerFnError> {
    Ok(())
}

pub fn list(_service: &str, _rel: &str) -> Vec<Entry> {
    Vec::new()
}

pub fn open_db() -> rusqlite::Result<Connection> {
    Connection::open_in_memory()
}

pub fn freeze<T>(f: impl FnOnce() -> T) -> T {
    f()
}

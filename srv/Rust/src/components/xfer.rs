// Dummy implementation for static file server

use leptos::prelude::*;
use server_fn::{
    ServerFnError,
    codec::{ByteStream, MultipartData},
};
use std::collections::HashMap;
use tokio::fs::File;

pub fn send(_filename: &str, _file: File) -> Result<ByteStream, ServerFnError> {
    Ok(ByteStream::new(futures::stream::empty::<Result<Vec<u8>, ServerFnError>>()))
}

pub async fn recv(
    _data: MultipartData,
    _max_bytes: u64,
    mut _on_file: impl FnMut(&str) -> Result<File, ServerFnError>,
) -> Result<(HashMap<String, String>, Option<(String, u64)>), ServerFnError> {
    Ok((HashMap::new(), None))
}

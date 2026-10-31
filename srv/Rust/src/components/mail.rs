#![cfg(feature = "ssr")]

use std::io;

// Dummy implementation for static file server
pub async fn send(_to: &str, _subject: &str, _body: &str) -> io::Result<()> {
    Ok(())
}

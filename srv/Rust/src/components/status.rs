// Dummy implementation for static file server

pub const SYSTEM_VERSION: &str = "0.1.0";

pub fn init() -> rusqlite::Result<()> {
    Ok(())
}

pub fn increment_access() {
    // No-op for static file server
}

pub fn access_counts() -> (i64, i64) {
    (0, 0)
}

pub fn flush() -> rusqlite::Result<()> {
    Ok(())
}

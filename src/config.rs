use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: [u8; 4],
    pub port: u16,
    pub workers: usize,
    pub timeout_ms: u64,

    pub analytics: Analytics,
}

#[derive(Debug, Deserialize)]
pub struct Analytics {
    pub database: PathBuf,
}

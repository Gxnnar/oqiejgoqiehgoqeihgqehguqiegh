use std::{fs, path::PathBuf};

use anyhow::Context;
use rusqlite::Connection;

use crate::{analytics::Analytics, config::Config};

pub struct App {
    pub config: Config,
    pub analytics: Analytics,
}

impl App {
    pub fn new(config_path: PathBuf) -> anyhow::Result<Self> {
        let config_raw = fs::read_to_string(config_path).context("Error loading config")?;
        let config = toml::from_str::<Config>(&config_raw).context("Error deserializing config")?;

        let database = Connection::open(&config.analytics.database)
            .context("Error opening database connection")?;
        let analytics = Analytics::new(database);
        analytics.init()?;

        Ok(Self { config, analytics })
    }
}

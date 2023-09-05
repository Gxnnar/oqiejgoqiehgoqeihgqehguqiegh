use std::{fs, path::PathBuf};

use rusqlite::Connection;

use crate::{analytics::Analytics, config::Config};

pub struct App {
    pub config: Config,
    pub analytics: Analytics,
}

impl App {
    pub fn new(config_path: PathBuf) -> anyhow::Result<Self> {
        let config_raw = fs::read_to_string(config_path)?;
        let config = toml::from_str::<Config>(&config_raw)?;

        let database = Connection::open(&config.analytics.database)?;
        let analytics = Analytics::new(database);
        analytics.init()?;

        Ok(Self { config, analytics })
    }
}

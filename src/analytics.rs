use std::{sync::Arc, time::Duration};

use afire::{internal::sync::ForceLockMutex, Request};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use rusqlite::{params, Connection};

pub struct Analytics {
    pub inner: Mutex<Option<Connection>>,
}

impl Analytics {
    pub fn new(database: Connection) -> Self {
        Self {
            inner: Mutex::new(Some(database)),
        }
    }

    fn take(&self) -> Connection {
        self.inner
            .lock()
            .take()
            .expect("Database connection is closed")
    }

    fn lock(&self) -> MappedMutexGuard<'_, Connection> {
        MutexGuard::map(self.inner.lock(), |x: &mut Option<Connection>| {
            x.as_mut().expect("Database connection is closed")
        })
    }

    pub fn init(&self) -> anyhow::Result<()> {
        let this = self.lock();

        // Set some pragmas
        this.pragma_update(None, "journal_mode", "WAL")?;
        this.pragma_update(None, "synchronous", "NORMAL")?;

        // Create tables
        this.execute(include_str!("./sql/create_requests.sql"), [])?;

        Ok(())
    }

    pub fn cleanup(&self) -> anyhow::Result<()> {
        let this = self.take();

        this.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        this.pragma_update(None, "optimize", "")?;
        this.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        drop(this);

        Ok(())
    }
}

impl Analytics {
    pub fn log_request(
        &self,
        request: &Arc<Request>,
        path: &str,
        response_status: u16,
        response_latency: Duration,
    ) -> anyhow::Result<()> {
        let headers = request
            .headers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let this = self.lock();
        this.execute(
            include_str!("./sql/insert_requests.sql"),
            params![
                request.socket.force_lock().peer_addr()?.to_string(),
                request.method.to_string(),
                path,
                request.version.to_string(),
                headers,
                request.body,
                response_status,
                response_latency.as_millis() as i64,
            ],
        )?;

        Ok(())
    }
}

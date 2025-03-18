//! Core functionality for the daemon library.
//!
//! This module defines the `Daemon` struct, which manages the daemon's state and lifecycle.

use std::sync::{Arc, Mutex};
use log::{info, error};
use crate::error::DaemonError;
use crate::config::DaemonConfig;
use crate::binary::BinaryManager;

/// Represents the state of the daemon.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DaemonState {
    Offline,
    Booting,
    Loading,
    Running,
    Restarting,
    Closing,
    Errored,
}

/// The main daemon struct.
pub struct Daemon {
    state: Arc<Mutex<DaemonState>>,
    binary_manager: Option<BinaryManager>,
}

impl Daemon {
    /// Creates a new daemon instance.
    pub fn new() -> Self {
        Daemon {
            state: Arc::new(Mutex::new(DaemonState::Offline)),
            binary_manager: None,
        }
    }

    /// Returns the current state of the daemon.
    pub fn get_state(&self) -> DaemonState {
        *self.state.lock().unwrap()
    }

    /// Starts the daemon with a synchronous callback.
    pub fn start(&self, on_start: impl FnOnce() -> Result<(), DaemonError>) -> Result<(), DaemonError> {
        let mut state = self.state.lock().unwrap();
        if *state == DaemonState::Running {
            return Err(DaemonError::AlreadyRunning);
        }

        *state = DaemonState::Booting;
        info!("Daemon is booting...");

        match on_start() {
            Ok(_) => {
                *state = DaemonState::Running;
                info!("Daemon is running.");
                Ok(())
            }
            Err(e) => {
                *state = DaemonState::Errored;
                error!("Failed to start daemon: {}", e);
                Err(e)
            }
        }
    }

    /// Stops the daemon with a synchronous callback.
    pub fn stop(&self, on_stop: impl FnOnce() -> Result<(), DaemonError>) -> Result<(), DaemonError> {
        let mut state = self.state.lock().unwrap();
        if *state != DaemonState::Running {
            return Err(DaemonError::NotRunning);
        }

        *state = DaemonState::Closing;
        info!("Daemon is closing...");

        match on_stop() {
            Ok(_) => {
                *state = DaemonState::Offline;
                info!("Daemon stopped successfully.");
                Ok(())
            }
            Err(e) => {
                *state = DaemonState::Errored;
                error!("Failed to stop daemon: {}", e);
                Err(e)
            }
        }
    }

    /// Restarts the daemon with a synchronous callback.
    pub fn restart(&self, on_restart: impl Fn() -> Result<(), DaemonError> + Clone) -> Result<(), DaemonError> {
        self.stop(on_restart.clone())?;
        self.start(on_restart)
    }

    /// Loads configuration into the daemon.
    pub fn load_config(&mut self, config: &DaemonConfig) -> Result<(), DaemonError> {
        // Set log level based on config
        log::set_max_level(match config.log_level.as_str() {
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        });

        // Load binaries
        let binary_manager = BinaryManager::new(config.binary_paths.clone());
        binary_manager.load_all()?;
        self.binary_manager = Some(binary_manager);

        Ok(())
    }

    /// Starts the daemon with an asynchronous callback (requires `async` feature).
    #[cfg(feature = "async")]
    pub async fn start_async<F, Fut>(&self, on_start: F) -> Result<(), DaemonError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), DaemonError>>,
    {
        let mut state = self.state.lock().unwrap();
        if *state == DaemonState::Running {
            return Err(DaemonError::AlreadyRunning);
        }

        *state = DaemonState::Booting;
        info!("Daemon is booting...");

        match on_start().await {
            Ok(_) => {
                *state = DaemonState::Running;
                info!("Daemon is running.");
                Ok(())
            }
            Err(e) => {
                *state = DaemonState::Errored;
                error!("Failed to start daemon: {}", e);
                Err(e)
            }
        }
    }

    /// Stops the daemon with an asynchronous callback (requires `async` feature).
    #[cfg(feature = "async")]
    pub async fn stop_async<F, Fut>(&self, on_stop: F) -> Result<(), DaemonError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), DaemonError>>,
    {
        let mut state = self.state.lock().unwrap();
        if *state != DaemonState::Running {
            return Err(DaemonError::NotRunning);
        }

        *state = DaemonState::Closing;
        info!("Daemon is closing...");

        match on_stop().await {
            Ok(_) => {
                *state = DaemonState::Offline;
                info!("Daemon stopped successfully.");
                Ok(())
            }
            Err(e) => {
                *state = DaemonState::Errored;
                error!("Failed to stop daemon: {}", e);
                Err(e)
            }
        }
    }
}
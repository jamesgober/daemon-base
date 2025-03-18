//! Error handling for the daemon library.
//!
//! This module defines the `DaemonError` enum, which represents errors that can occur
//! during the daemon's lifecycle.

use thiserror::Error;

/// Represents errors that can occur in the daemon.
#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("Daemon is already running")]
    AlreadyRunning,
    #[error("Daemon is not running")]
    NotRunning,
    #[error("Custom error: {0}")]
    CustomError(String),
}
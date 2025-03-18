//! Configuration management for the daemon library.
//!
//! This module defines the `DaemonConfig` struct, which is used to load and manage
//! configuration settings for the daemon.

use serde::{Deserialize, Serialize};
use std::fs;

/// Represents the configuration for the daemon.
#[derive(Debug, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// The log level (e.g., "debug", "info", "warn", "error").
    pub log_level: String,
    /// Paths to binaries or resources to load.
    pub binary_paths: Vec<String>,
    /// Whether async support is enabled.
    pub async_enabled: bool,
}

impl DaemonConfig {
    /// Loads the configuration from a file.
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: DaemonConfig = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}
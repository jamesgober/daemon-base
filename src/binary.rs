//! Binary management for the daemon library.
//!
//! This module provides functionality to load and unload binaries or resources
//! in a specified order.

use log::{info};
use std::path::Path;
use std::process::Command;
use crate::error::DaemonError;

/// Represents a binary or resource to be loaded.
pub struct Binary {
    path: String,
}

impl Binary {
    /// Creates a new `Binary` instance.
    pub fn new(path: String) -> Self {
        Binary { path }
    }

    /// Loads the binary or resource.
    pub fn load(&self) -> Result<(), DaemonError> {
        info!("Loading binary: {}", self.path);

        if !Path::new(&self.path).exists() {
            return Err(DaemonError::CustomError(format!("Binary not found: {}", self.path)));
        }

        // Example: Execute the binary (replace with actual logic)
        let output = Command::new(&self.path)
            .output()
            .map_err(|e| DaemonError::CustomError(format!("Failed to execute binary: {}", e)))?;

        if !output.status.success() {
            return Err(DaemonError::CustomError(format!(
                "Binary failed with output: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        info!("Successfully loaded binary: {}", self.path);
        Ok(())
    }

    /// Unloads the binary or resource.
    pub fn unload(&self) -> Result<(), DaemonError> {
        info!("Unloading binary: {}", self.path);

        // Example: Kill the process (replace with actual logic)
        let output = Command::new("pkill")
            .arg(&self.path)
            .output()
            .map_err(|e| DaemonError::CustomError(format!("Failed to unload binary: {}", e)))?;

        if !output.status.success() {
            return Err(DaemonError::CustomError(format!(
                "Failed to unload binary: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        info!("Successfully unloaded binary: {}", self.path);
        Ok(())
    }
}

/// Manages a list of binaries or resources.
pub struct BinaryManager {
    binaries: Vec<Binary>,
}

impl BinaryManager {
    /// Creates a new `BinaryManager` instance.
    pub fn new(binaries: Vec<String>) -> Self {
        BinaryManager {
            binaries: binaries.into_iter().map(Binary::new).collect(),
        }
    }

    /// Loads all binaries or resources.
    pub fn load_all(&self) -> Result<(), DaemonError> {
        for binary in &self.binaries {
            binary.load()?;
        }
        Ok(())
    }

    /// Unloads all binaries or resources.
    pub fn unload_all(&self) -> Result<(), DaemonError> {
        for binary in &self.binaries {
            binary.unload()?;
        }
        Ok(())
    }
}
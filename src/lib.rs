//! A flexible and configurable daemon library for Rust.
//!
//! This library provides a daemon (background service) that can be easily integrated into
//! various projects. It supports both synchronous and asynchronous operations, lifecycle
//! callbacks, and configuration management.

pub mod core;
pub mod config;
pub mod error;
pub mod logging;
pub mod binary; // Binary management module
pub mod concurrency; // Concurrency models module

pub use core::{Daemon, DaemonState};
pub use config::DaemonConfig;
pub use error::DaemonError;
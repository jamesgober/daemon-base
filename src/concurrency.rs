//! Concurrency models for the daemon library.
//!
//! This module provides functionality to support both multi-threaded and
//! asynchronous operations.

use std::thread;
use log::info;
use crate::error::DaemonError;

/// Represents a concurrency model.
pub enum ConcurrencyModel {
    MultiThreaded,
    Async,
}

/// Executes a task using the specified concurrency model.
pub fn execute_task<F>(model: ConcurrencyModel, task: F) -> Result<(), DaemonError>
where
    F: FnOnce() -> Result<(), DaemonError> + Send + 'static,
{
    match model {
        ConcurrencyModel::MultiThreaded => {
            info!("Executing task in multi-threaded mode.");
            let handle = thread::spawn(move || {
                if let Err(e) = task() {
                    log::error!("Task failed: {}", e);
                }
            });
            handle.join().map_err(|_| DaemonError::CustomError("Thread panicked".to_string()))?;
        }
        ConcurrencyModel::Async => {
            #[cfg(feature = "async")]
            {
                info!("Executing task in async mode.");
                let runtime = tokio::runtime::Runtime::new()
                    .map_err(|e| DaemonError::CustomError(format!("Failed to create runtime: {}", e)))?;
                runtime.block_on(async {
                    if let Err(e) = task() {
                        log::error!("Task failed: {}", e);
                    }
                });
            }
            #[cfg(not(feature = "async"))]
            {
                return Err(DaemonError::CustomError("Async feature is not enabled".to_string()));
            }
        }
    }
    Ok(())
}
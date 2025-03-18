//! Example usage of the daemon library.
//!
//! This example demonstrates how to use the `Daemon` struct to manage a daemon's lifecycle,
//! load configuration, and execute tasks using different concurrency models.

use daemon_base::core::{Daemon};
use daemon_base::config::DaemonConfig;
use daemon_base::concurrency::{ConcurrencyModel, execute_task};
use log::info;

fn main() {
    // Initialize logging
    daemon_base::logging::init_logging().unwrap();

    // Create a new daemon instance
    let mut daemon = Daemon::new();

    // Load configuration
    let config = DaemonConfig::from_file("config.json").unwrap();
    daemon.load_config(&config).unwrap();

    // Start the daemon with a task
    daemon.start(|| {
        info!("Custom start logic executed.");
        execute_task(ConcurrencyModel::MultiThreaded, || {
            info!("Task executed in multi-threaded mode.");
            Ok(())
        })
    }).unwrap();

    println!("Daemon state: {:?}", daemon.get_state());

    // Stop the daemon
    daemon.stop(|| {
        info!("Custom stop logic executed.");
        Ok(())
    }).unwrap();

    println!("Daemon state: {:?}", daemon.get_state());
}
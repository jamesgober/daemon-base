//! Unit tests for the core functionality of the daemon library.

use daemon_lib::core::{Daemon, DaemonState};
use daemon_lib::error::DaemonError;

#[test]
fn test_daemon_lifecycle() {
    let daemon = Daemon::new();

    // Test start
    assert_eq!(daemon.get_state(), DaemonState::Offline);
    daemon.start(|| Ok(())).unwrap();
    assert_eq!(daemon.get_state(), DaemonState::Running);

    // Test stop
    daemon.stop(|| Ok(())).unwrap();
    assert_eq!(daemon.get_state(), DaemonState::Offline);

    // Test error handling
    let result = daemon.start(|| Err(DaemonError::CustomError("Test error".to_string())));
    assert!(result.is_err());
    assert_eq!(daemon.get_state(), DaemonState::Errored);
}
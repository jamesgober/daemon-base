<h1 align="center">
    Daemon Library
</h1>

<p align="center">
    <a href="https://crates.io/crates/daemon-base"><img alt="Crates.io" src="https://img.shields.io/crates/v/daemon-base"></a>
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/jamesgober/daemon-base?color=%23347d39" alt="last commit badge">
    <img alt="GitHub" src="https://img.shields.io/github/license/jamesgober/daemon-base">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/daemon-base">
</p>

**Daemon Base** is a flexible and configurable Rust daemon library for managing background services. It supports lifecycle management, logging, callbacks, and optional async execution. Designed for cross-platform use, it runs on Linux, macOS, and Windows, making it ideal for system services, background tasks, and process management.



## Features

- **Cross-Platform** – Works seamlessly on Linux, macOS, and Windows.
- **Lifecycle Management** – Start, stop, restart, and monitor the daemon’s state.
- **Event Callbacks** – Hook into lifecycle events to define custom behavior.
- **Configurable** – Load settings from a JSON file for easy customization.
- **Logging & Error Handling** – Built-in structured logging with configurable levels.
- **Optional Async Support** – Supports both synchronous and asynchronous execution (Tokio).
- **Graceful Shutdown** – Ensures safe cleanup and state persistence on exit.
- **Process Control** – Manage background tasks efficiently with threading support.

&nbsp;

---

## Usage

Add the library to your `Cargo.toml`:
To use **daemon-base**, add the following to your `Cargo.toml`:
```toml
[dependencies]
daemon-base = "0.1.1"
```
&nbsp;

OR From GitHub (Latest Version)
```toml
[dependencies]
daemon-base = { git = "https://github.com/jamesgober/daemon-base", branch = "main" }
```

&nbsp;


&nbsp;

## Usage

### 1. Basic Daemon Setup
This example initializes a simple daemon and starts it.
```rust
use daemon_base::{Daemon, DaemonState};

fn main() {
    let daemon = Daemon::new();

    println!("Daemon is in state: {:?}", daemon.get_state());

    daemon.start();
    println!("Daemon started. Current state: {:?}", daemon.get_state());
}
```

*Expected Output:*
```
Daemon is in state: Offline
Daemon started. Current state: Running
```

&nbsp;

### 2. Handling Lifecycle Events (Callbacks)
Customize daemon behavior by hooking into lifecycle events.
```rust
use daemon_base::{Daemon, DaemonState};

fn on_start() {
    println!("Daemon is starting...");
}

fn on_shutdown() {
    println!("Daemon is shutting down.");
}

fn main() {
    let mut daemon = Daemon::new();

    daemon.on_start(on_start);
    daemon.on_shutdown(on_shutdown);

    daemon.start();
    daemon.stop();
}
```


*Expected Output:*
```
Daemon is starting...
Daemon is shutting down.
```

&nbsp;

### 3. Configuring the Daemon with JSON
You can load a configuration file instead of hardcoding values.

config.json
```json
{
    "log_level": "info",
    "multi_threading": true,
    "extensions": ["frag-indexer", "query-optimizer"]
}

```

Rust Code
```rust
use daemon_base::DaemonConfig;

fn main() {
    let config = DaemonConfig::load("config.json").expect("Failed to load config");
    println!("Loaded configuration: {:?}", config);
}
```


*Expected Output:*
```
Loaded configuration: DaemonConfig { log_level: "info", multi_threading: true, extensions: ["frag-indexer", "query-optimizer"] }

```

&nbsp;

### 4. Running the Daemon as a Background Process
To run the daemon in the background and detach from the terminal:
```rust
use daemon_base::Daemon;

fn main() {
    let daemon = Daemon::new();
    daemon.start_detached();
    println!("Daemon started in the background.");
}

```
**Output:** *(Daemon keeps running in the background, no terminal output after start.)*

&nbsp;

### 5. Enabling Asynchronous Mode
If you're using Tokio async runtime, enable the `async` feature in` Cargo.toml`:
```toml
[dependencies]
daemon-base = { version = "0.1.1", features = ["async"] }
tokio = { version = "1", features = ["full"] }
```

Then, use `async` methods in Rust:
```rust
use daemon_base::Daemon;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let daemon = Daemon::new();

    daemon.start_async().await;
    println!("Daemon started asynchronously.");

    sleep(Duration::from_secs(5)).await;

    daemon.stop_async().await;
    println!("Daemon stopped.");
}
```
**Output:** *(Runs asynchronously for 5 seconds before shutting down.)*


---

&nbsp;

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

---

*Copyright &copy; 2025 James Gober.*   
https://jamesgober.com
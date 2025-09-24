//! Daemon logic
//!
//! Implementation of:
//! - Persistent daemon process
//! - File system watching for change events
//! - Event-driven execution of lint/format commands
//! - Configurable `config.toml` loading

use chrono::Local;
use daemonize::Daemonize;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use std::{fs, fs::File, io::Write, process::Command, sync::mpsc::channel, time::Duration};

/// Top-level configuration parsed from `config.toml`
#[derive(Deserialize)]
pub struct Config {
    /// Daemon-related configuration
    pub daemon: DaemonConfig,
}

/// Configuration options for the daemon
#[derive(Deserialize)]
pub struct DaemonConfig {
    /// Path to the log file where daemon writes messages
    pub log_file: String,
    /// Directory path to watch for file changes
    pub watch_path: String,
    /// Shell command executed when linting is triggered
    pub lint_command: String,
    /// Shell command executed when formatting is triggered
    pub format_command: String,
}

/// Load TOML configuration from disk
///
/// # Panics
/// Will panic if the file cannot be read or parsed.
///
/// # Example
/// ```no_run
/// use core_daemon::daemon_logic::d_logic::load_config;
/// let config = load_config("config.toml");
/// ```
pub fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path).expect("Failed to read config.toml");
    toml::from_str(&content).expect("Failed to parse config.toml")
}

/// Run the daemon in the background.
///
/// This will:
/// - Fork the process
/// - Detach from the terminal
/// - Start watching the configured directory
///
/// # Example
/// ```no_run
/// use core_daemon::daemon_logic::d_logic::{load_config, run_daemon};
/// let config = load_config("config.toml");
/// run_daemon(config);
/// ```
pub fn run_daemon(config: Config) {
    let stdout = File::create("/tmp/core_daemon.out").expect("Failed to create stdout file");
    let stderr = File::create("/tmp/core_daemon.err").expect("Failed to create stderr file");

    let daemonize = Daemonize::new().stdout(stdout).stderr(stderr);

    daemonize.start().expect("Failed to daemonize");

    start_file_watcher(config);
}

/// Write a message to the daemon log file with timestamp
fn log_message(log_file: &str, msg: &str) {
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(log_file)
        .expect("Unable to open log file");
    let entry = format!("{}: {}\n", Local::now(), msg);
    file.write_all(entry.as_bytes())
        .expect("Failed to write log");
}

/// Start the file watcher loop.
///
/// Spawns a watcher on the configured path and listens for file system events.
fn start_file_watcher(config: Config) {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .expect("Failed to create watcher");

    watcher
        .watch(config.daemon.watch_path.as_ref(), RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    log_message(&config.daemon.log_file, "Started watching directory...");

    for res in rx {
        match res {
            Ok(event) => handle_event(&config, event),
            Err(e) => log_message(&config.daemon.log_file, &format!("Watcher error: {:?}", e)),
        }
    }
}

/// Handle file system events by logging them and spawning lint/format commands.
fn handle_event(config: &Config, event: Event) {
    for path in event.paths {
        log_message(
            &config.daemon.log_file,
            &format!("Change detected: {:?}", path),
        );

        // Run lint command
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&config.daemon.lint_command)
            .spawn();

        // Run format command
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&config.daemon.format_command)
            .spawn();
    }
}

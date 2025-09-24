use std::fs::File;
use daemonize::Daemonize;
use crate::config::Config;
use crate::watcher::start_file_watcher;
use crate::logger::{log_message, init_log};

pub fn run_daemon(config: Config, debug_mode: bool) {
    init_log(&config.daemon.log_file);

    if !debug_mode {
        let stdout = File::create("/tmp/lintpulse.out").expect("Failed to create stdout file");
        let stderr = File::create("/tmp/lintpulse.err").expect("Failed to create stderr file");

        let daemonize = Daemonize::new().stdout(stdout).stderr(stderr);
        daemonize.start().expect("Failed to daemonize process");
    }

    log_message(&config.daemon.log_file, "Daemon started successfully");

    start_file_watcher(
        config.daemon.watch_path,
        config.daemon.log_file,
        config.rules,
        debug_mode,
    );
}

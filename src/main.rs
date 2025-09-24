mod config;
mod logger;
mod commands;
mod watcher;
mod daemon;

use std::path::Path;
use config::load_config;
use daemon::run_daemon;

fn main() {
    println!("Starting LintPulse daemon...");

    let config_path = "config.toml";
    if !Path::new(config_path).exists() {
        println!("Config file not found, creating default config.toml...");

        let default_config = r#"
[daemon]
log_file = "lintpulse.log"
watch_path = "./"

[[rules]]
pattern = "*.rs"
lint_command = "cargo clippy --quiet"
format_command = "rustfmt {file}"

[[rules]]
pattern = "*.js"
lint_command = "eslint {file}"
format_command = "prettier --write {file}"

[[rules]]
pattern = "*.py"
lint_command = "flake8 {file}"
format_command = "black {file}"
"#;

        std::fs::write(config_path, default_config).expect("Failed to write default config.toml");
        println!("Default config.toml created.");
    }

    let config = load_config(config_path);
    run_daemon(config, true); // debug_mode = true
}

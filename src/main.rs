mod daemon_start;
use std::{fs, path::Path};
use daemon_start::{load_config, run_daemon};

fn main() {
    println!("Starting LintPulse daemon...");

    let config_path = "config.toml";
    if !Path::new(config_path).exists() {
        println!("Config file not found, creating default config.toml...");

        let default_config = r#"
[daemon]
log_file = "lintpulse.log"
watch_path = "./"
lint_command = "echo linting {file}"
format_command = "echo formatting {file}"
"#;

        fs::write(config_path, default_config).expect("Failed to write default config.toml");
        println!("Default config.toml created.");
    }

    let config = load_config(config_path);
    run_daemon(config);
}

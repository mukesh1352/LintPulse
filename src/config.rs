use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub daemon: DaemonConfig,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Clone)]
pub struct DaemonConfig {
    pub log_file: String,
    pub watch_path: String,
}

#[derive(Deserialize, Clone)]
pub struct Rule {
    pub pattern: String,         // e.g., "*.rs", "*.js"
    pub lint_command: String,    // e.g., "cargo clippy {file}"
    pub format_command: String,  // e.g., "cargo fmt {file}"
}

pub fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path).expect("Failed to read config.toml");
    toml::from_str(&content).expect("Failed to parse config.toml")
}

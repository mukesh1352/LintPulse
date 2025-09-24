# Incremental Formatter & Linter Daemon

A persistent daemon that watches your codebase and automatically formats and lints only the files that change. Supports plugins for any language using WebAssembly.

---

## Setup

### Prerequisites

- Rust programming language installed
- WASM runtime like Wasmtime installed
- Git installed

---

### Rust Commands

- `cargo build` — Build the Rust application
- `cargo run` — Run the Rust application
- `cargo check` — Quickly check your code for issues before deployment

---

### How to Use the Makefile

Use the following commands to build, run, test, and clean your project using the Makefile:

- **Build the project:**
```bash
make build
```
- **Run the project:**
```bash
make run
```

- **Clean build artifacts:**

```bash
make clean
```

### NOTE:

---

### Notes

- On first run, if `config.toml` is missing, the daemon will create a default configuration file and then proceed without requiring a restart.
- Update your `config.toml` as needed to customize log file location, watch path, and lint/format commands.
- Logs are written to the file specified in the config (`log_file`), capturing file change events and command executions.
- To stop the daemon, use `Ctrl+C` or appropriate OS signal.

---
*For documentation purpose
```bash
cargo doc --open
```
logger.rs handles logging

watcher.rs handles file watching and debounce logic

commands.rs handles async lint/format execution

daemon.rs handles daemonization

config.rs handles configuration
src/
├─ main.rs         # Entry point
├─ config.rs       # Load config.toml
├─ logger.rs       # Thread-safe logging
├─ commands.rs     # Async lint/format execution
├─ watcher.rs      # File watcher with debounce/throttle and per-file rules
└─ daemon.rs       # Daemonization logic


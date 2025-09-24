# LintPulse

**LintPulse** is a lightweight, incremental file-watching daemon designed for developers. It automatically runs linters and formatters on files as they change, providing instant feedback. LintPulse supports Rust, JavaScript, and Python out of the box, and can be easily extended to other languages via configuration.

---

## Features

- Incremental file watcher daemon.
- Supports Rust (`cargo clippy`, `cargo fmt`), JavaScript (`eslint`, `prettier`), and Python (`flake8`, `black`).
- Logs file changes and command executions to a configurable log file (`lintpulse.log` by default).
- Debounced processing prevents redundant executions for rapidly changing files.
- Fully configurable via `config.toml`.
- Modular design: simple to extend with new languages or tools.

---

## Setup

### Rust Commands

- **Build the project:**
```bash
cargo build
```
-**Run the project**
```bash
cargo run
```

### Using the makefile
-**Build the project**
```bash
make build
```
-**Run the project**
```bash
make run
```
-**Clean Build Artifact**
```bash
make clean
```

## Notes
- On first run, if config.toml is missing, the daemon automatically creates a default configuration file.
- Customize config.toml to adjust the log file location, watch path, and lint/format commands.
- Logs capture file change events and executed commands.
- To stop the daemon, use Ctrl+C or an appropriate OS signal.
- For documentation, you can generate and open it with:
```bash
cargo doc --open
```


## Project Structure
src/
├─ main.rs         # Entry point
├─ config.rs       # Load and parse config.toml
├─ logger.rs       # Thread-safe logging
├─ commands.rs     # Async execution of lint/format commands
├─ watcher.rs      # File watcher with debounce and per-file rules
└─ daemon.rs       # Daemonization logic

# LintPulse

**LintPulse** is a lightweight, incremental file-watching daemon tailored for developers. It automatically runs linters and formatters on files as they change, providing instant feedback to streamline the development workflow. Out of the box, LintPulse supports Rust, JavaScript, and Python, and can be easily extended to other languages through configuration.

---

## Features

- Incremental file watcher daemon, optimized for performance.
- Built-in support for Rust (`cargo clippy`, `cargo fmt`), JavaScript (`eslint`, `prettier`), and Python (`flake8`, `black`).
- Detailed logging of file changes and command executions to a configurable log file (`lintpulse.log` by default).
- Debounce mechanism to prevent redundant executions during rapid file changes.
- Fully configurable via a user-friendly `config.toml`.
- Modular and extensible design, enabling easy integration of new languages and tools.

---

## Setup

### Prerequisites

- Rust programming language installed  
- WASM runtime (e.g., Wasmtime) installed (for plugin support)  
- Git installed  

### Rust Commands

- **Build the project:**
```bash
cargo build
```

- **Run the project:**
```bash
cargo run
```


### Using the Makefile

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


---

## Notes

- On first run, if `config.toml` is missing, the daemon automatically generates a default configuration file.
- Customize `config.toml` to modify the log file path, watched directories, and lint/format commands to fit your project needs.
- Logs comprehensively capture all file change events and executed commands for auditing and debugging.
- To stop the daemon, use `Ctrl+C` or send the appropriate termination signal for your operating system.
- For comprehensive API documentation, generate and view it using:

```bash
cargo doc --open
```


---

## Project Structure

```shell
src/
├─ main.rs # Application entry point
├─ config.rs # Configuration loading and parsing logic
├─ logger.rs # Thread-safe logging utilities
├─ commands.rs # Asynchronous execution of linting and formatting commands
├─ watcher.rs # File system watcher with debouncing and per-file processing rules
└─ daemon.rs # Daemonization and process management
```

## Upcoming Development Roadmap
LintPulse continues to evolve with exciting features planned to enhance extensibility, usability, and developer experience:
 [ ] Design and document a robust plugin interface for WebAssembly (WASM) modules, enabling language-agnostic linting/formatting plugins.  
- [ ] Integrate a WASM runtime to dynamically load and execute plugins securely and efficiently.  
- [ ] Develop example WASM plugins to demonstrate linting and formatting capabilities.  
- [ ] Implement comprehensive plugin management for loading, unloading, and error handling.

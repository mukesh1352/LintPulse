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
### Why forking is important?
Daemon processes are meant to run independently of the terminal
.Detachment from terminal
If you don’t fork, when the user closes the terminal, the daemon would also stop. Forking allows it to run in the background.

Run in the background continuously
Background tasks like watching a directory for file changes or running lint/format commands need to stay alive indefinitely.

Redirect output
Daemons typically write logs to files (/tmp/core_daemon.out and .err in your code) instead of stdout/stderr, because there’s no terminal attached.


Separation from terminal → more robust

Without forking:

Your process is attached to the terminal.

Closing the terminal kills the process.

Any input/output goes to the terminal.

With forking:

The daemon detaches completely.

It runs independently of the user session.

Logs go to files, which is more predictable and manageable.

💡 Efficiency gain: The OS doesn’t need to keep your process tied to a shell session, so the daemon can run uninterrupted.

2️⃣ Background execution → non-blocking

Forking allows your daemon to run in the background:

You can start it once and forget about it.

It can continuously watch files and execute lint/format commands without user interaction.

No terminal resources are consumed.

💡 Efficiency gain: CPU and memory are only used for the actual tasks, not for maintaining a terminal or UI.

3️⃣ Proper signal handling → safer resource management

Daemons handle UNIX signals (like SIGHUP, SIGTERM) differently:

They can clean up temporary files or gracefully stop the watcher.

Forking ensures the parent exits and the child runs with its own session ID, avoiding accidental termination by terminal signals.

💡 Efficiency gain: Reduces crashes, avoids orphaned processes, and ensures log consistency.

4️⃣ Logging and resource isolation

By redirecting stdout/stderr to files:

Logs don’t flood the terminal.

Multiple daemons can run without interfering with each other.

You avoid clutter and accidental blocking of terminal I/O.

💡 Efficiency gain: File-based logging is faster and more reliable than printing to terminal repeatedly.

5️⃣ Scalability

When your daemon forks:

You can easily spawn multiple background processes if needed.

Each can handle different directories or tasks.

Makes the system modular and maintainable.

💡 Efficiency gain: Supports larger projects or multiple watchers without blocking the main terminal session.

⚖️ Trade-offs

Forking slightly increases initial startup complexity.

Debugging is harder because the process runs in the background.

But for long-running, event-driven tasks, the advantages outweigh these minor costs.

use std::process::Command;
use crate::logger::log_message;

pub fn run_commands_async(log_file: String, lint_cmd: String, format_cmd: String) {
    // Lint
    let log_clone = log_file.clone();
    let lint_clone = lint_cmd.clone();
    std::thread::spawn(move || {
        match Command::new("sh").arg("-c").arg(&lint_clone).spawn() {
            Ok(_) => log_message(&log_clone, &format!("Lint executed: {}", lint_clone)),
            Err(e) => log_message(&log_clone, &format!("Lint failed: {:?}, {}", e, lint_clone)),
        }
    });

    // Format
    let log_clone2 = log_file.clone();
    let format_clone = format_cmd.clone();
    std::thread::spawn(move || {
        match Command::new("sh").arg("-c").arg(&format_clone).spawn() {
            Ok(_) => log_message(&log_clone2, &format!("Format executed: {}", format_clone)),
            Err(e) => log_message(&log_clone2, &format!("Format failed: {:?}, {}", e, format_clone)),
        }
    });
}

use crate::commands::run_commands_async;
use crate::config::Rule;
use crate::logger::log_message;
use glob::Pattern;
use notify::{Config as NotifyConfig, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

fn should_ignore(path: &str, log_file: &str) -> bool {
    path.contains("/target/")
        || path.contains("/.git/")
        || path.contains("/node_modules/")
        || path.ends_with(".lock")  // cargo/npm lockfiles
        || path.ends_with("~")      // temp files
        || path.ends_with(log_file) // ignore the log file itself
}

pub fn start_file_watcher(
    watch_path: String,
    log_file: String,
    rules: Vec<Rule>,
    debug_mode: bool,
) {
    let changed_files: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    let last_modified: Arc<Mutex<HashMap<String, Instant>>> = Arc::new(Mutex::new(HashMap::new()));
    let debounce_duration = Duration::from_millis(500);

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, NotifyConfig::default()).expect("Failed to create watcher");

    watcher
        .watch(Path::new(&watch_path), RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    let log_file_arc = Arc::new(log_file);
    log_message(
        &log_file_arc,
        &format!("Watching directory: {}", watch_path),
    );

    let changed_files_thread = Arc::clone(&changed_files);
    let last_modified_thread = Arc::clone(&last_modified);
    let rules_thread = rules.clone();
    let log_file_thread = Arc::clone(&log_file_arc);

    // Debounce + async processing
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));

        let mut files = changed_files_thread.lock().unwrap();
        if files.is_empty() {
            continue;
        }

        let now = Instant::now();
        let mut last_mod = last_modified_thread.lock().unwrap();
        let mut processed_files = Vec::new();

        for file in files.iter() {
            let elapsed = last_mod
                .get(file)
                .map_or(debounce_duration, |t| now.duration_since(*t));
            if elapsed >= debounce_duration {
                processed_files.push(file.clone());
            }
        }

        for file in &processed_files {
            last_mod.insert(file.clone(), now);

            let log_clone = Arc::clone(&log_file_thread);
            if debug_mode {
                println!("Processing file: {}", file);
            }
            log_message(&log_clone, &format!("Processing file: {}", file));

            // Find matching rule for this file
            for rule in &rules_thread {
                if Pattern::new(&rule.pattern).unwrap().matches(file) {
                    let lint_cmd = rule.lint_command.replace("{file}", file);
                    let format_cmd = rule.format_command.replace("{file}", file);
                    run_commands_async(log_clone.as_ref().to_string(), lint_cmd, format_cmd);
                    break; // stop after first matching rule
                }
            }
        }

        for file in processed_files {
            files.remove(&file);
        }
    });

    // Event loop
    for res in rx {
        match res {
            Ok(event) => handle_event(
                &event,
                &changed_files,
                &last_modified,
                &log_file_arc,
                debug_mode,
            ),
            Err(e) => log_message(&log_file_arc, &format!("Watcher error: {:?}", e)),
        }
    }
}

fn handle_event(
    event: &Event,
    changed_files: &Arc<Mutex<HashSet<String>>>,
    last_modified: &Arc<Mutex<HashMap<String, Instant>>>,
    log_file: &Arc<String>,
    debug_mode: bool,
) {
    for path in &event.paths {
        if let Some(path_str) = path.to_str() {
            if should_ignore(path_str, log_file) {
                continue; // skip irrelevant files and the log file
            }

            if debug_mode {
                println!("File changed: {}", path_str);
            }
            log_message(log_file, &format!("File changed: {}", path_str));

            let mut files = changed_files.lock().unwrap();
            files.insert(path_str.to_string());

            let mut last_mod = last_modified.lock().unwrap();
            last_mod.insert(path_str.to_string(), Instant::now());
        }
    }
}

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;
use std::path::PathBuf;

fn main() -> notify::Result<()> {
    // Read the first command-line argument as the path to watch
    let args: Vec<String> = env::args().collect();
    let path_to_watch = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        eprintln!("Usage: lintpulse <path-to-watch>");
        std::process::exit(1);
    };

    // Check if the path exists and is a directory
    if !path_to_watch.exists() || !path_to_watch.is_dir() {
        eprintln!("Error: Path {:?} does not exist or is not a directory", path_to_watch);
        std::process::exit(1);
    }

    // Create channel for events
    let (tx, rx) = channel();

    // Create watcher with 500ms debounce
    let mut watcher = watcher(tx, Duration::from_millis(500))?;

    // Start recursive watching
    watcher.watch(&path_to_watch, RecursiveMode::Recursive)?;

    println!("Daemon started. Watching for changes in {:?}", path_to_watch);

    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Create(path)
                    | DebouncedEvent::Write(path)
                    | DebouncedEvent::Remove(path)
                    | DebouncedEvent::Rename(_, path) => {
                        println!("File changed: {:?}", path);
                        // TODO: Trigger lint/format on changed file
                    }
                    _ => {}
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

mod handle_builder;
mod handle_fetcher;

use notify::{Config,
             RecommendedWatcher,
             RecursiveMode,
             Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_dir = std::env::var("REPO_DIR").unwrap_or_else(|_| ".".to_owned());
    let repo_path = Path::new(&repo_dir);

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default()).unwrap();
    watcher.watch(repo_path, RecursiveMode::Recursive)
           .unwrap();

    let repo_dir_clone = repo_dir.clone();
    let handle1 = thread::spawn(move || {
        handle_fetcher::run(rx, repo_dir_clone);
    });
    let handle2 = thread::spawn(move || {
        handle_builder::run(repo_dir);
    });

    handle1.join()
           .unwrap();
    handle2.join()
           .unwrap();

    Ok(())
}

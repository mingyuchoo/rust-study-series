use notify::{Error,
             Event};
use std::{process::Command,
          sync::mpsc::Receiver};

pub fn run(rx: Receiver<Result<Event, Error>>,
           repo_dir: String) {
    loop {
        match rx.recv() {
            | Ok(Ok(event)) => {
                let is_in_target_dir = event.paths
                                            .iter()
                                            .any(|path| {
                                                path.strip_prefix(&repo_dir)
                            .ok()
                            .and_then(|p| {
                                p.components()
                                    .next()
                            })
                            == Some(std::path::Component::Normal(std::ffi::OsStr::new("target")))
                                            });

                if is_in_target_dir {
                    continue; // Ignore events in the 'target' directory
                }

                println!("File change detected! Running 'git pull'...");
                let output = Command::new("git").arg("pull")
                                                .current_dir(&repo_dir)
                                                .output()
                                                .expect("Failed to execute git pull");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            | Ok(Err(e)) => println!("watch error: {:?}", e),
            | Err(e) => println!("watch error: {:?}", e),
        }
    }
}

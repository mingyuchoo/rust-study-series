use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

pub fn run(repo_dir: String) {
    let build_command = std::env::var("BUILD_COMMAND").unwrap_or_else(|_| "make build".to_owned());

    loop {
        println!("Running build command ...");

        let mut command_parts = build_command.split_whitespace();
        let command = command_parts.next().unwrap();
        let args: Vec<&str> = command_parts.collect();

        let output = Command::new(command)
            .args(&args)
            .current_dir(&repo_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to execute build command");
        println!("{}", String::from_utf8_lossy(&output.stdout));

        thread::sleep(Duration::from_secs(5));
    }
}

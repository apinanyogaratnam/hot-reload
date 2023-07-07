use std::{collections::HashMap, process::{Command, exit, Child}, thread, time::Duration};
use walkdir::WalkDir;
use std::os::unix::process::CommandExt;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

fn main() {
    let mut files = HashMap::new();
    let mut child = start_server();

    loop {
        let mut changed = false;
        for entry in WalkDir::new(".") {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "go") {
                let metadata = path.metadata().unwrap();
                let mtime = metadata.modified().unwrap();
                if files.get(path) != Some(&mtime) {
                    println!("File {:?} has been modified. Reloading...", path);
                    files.insert(path.to_path_buf(), mtime);
                    changed = true;
                }
            }
        }

        if changed {
            // Send SIGKILL signal to child process
            kill(Pid::from_raw(child.id() as i32), Signal::SIGKILL).unwrap();
            child.wait().unwrap();
            child = start_server();
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn start_server() -> Child {
    Command::new("make")
        .arg("start")
        .exec() // exec replaces the current process with the new one
}

use std::{collections::HashMap, process::Command, thread, time::Duration};
use walkdir::WalkDir;
use std::process::Child;
use std::os::unix::prelude::CommandExt;
use nix::unistd::{Pid, getpid, setpgid};
use nix::sys::signal::{killpg, Signal};

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
            let pid = Pid::from_raw(child.id() as i32);
            // Send SIGTERM signal to the process group of the child process
            killpg(pid, Signal::SIGTERM).expect("Failed to kill process group");
            child.wait().unwrap();
            child = start_server();
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn start_server() -> Child {
    let mut command = Command::new("make");
    command.arg("start");
    unsafe {
        command.pre_exec(|| {
            let pid = getpid();
            // Create a new process group
            setpgid(pid, pid).expect("Failed to create a new process group");
            Ok(())
        });
    }
    command.spawn().expect("Failed to start the server")
}

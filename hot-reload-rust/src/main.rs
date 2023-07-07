use std::{collections::HashMap, path::Path, process::Command, thread, time::Duration};
use walkdir::WalkDir;
use std::process::Child;
use std::process;
use std::os::unix::process::CommandExt;

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
                    files.insert(Path::new(path).to_path_buf(), mtime);
                    changed = true;
                }
            }
        }

        if changed {
            child.kill().unwrap();
            child = start_server();
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn start_server() -> Child {
    Command::new("make")
        .arg("start")
        .spawn()
        .expect("Failed to start the server")
}

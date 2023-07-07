use std::{collections::HashMap, thread, time::Duration};
use walkdir::WalkDir;
use subprocess::{Popen, PopenConfig};

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
            child.kill().unwrap();
            child.wait().unwrap();
            child = start_server();
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn start_server() -> Popen {
    Popen::create(&["make", "start"], PopenConfig::default())
        .expect("Failed to start the server")
}

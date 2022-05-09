use std::{env::current_dir, path::PathBuf};

use regex::Regex;

fn read_dir(dir: PathBuf) -> Vec<PathBuf> {
    let r = Regex::new(r"\.(c|h){1,2}(pp)?$").unwrap();
    let mut files = Vec::<PathBuf>::new();

    if let Ok(entries) = dir.read_dir() {
        for entry in entries.flatten() {
            let path = entry.path();
            if entry.metadata().unwrap().is_dir() {
                files.append(&mut read_dir(path));
            } else if r.is_match(path.to_string_lossy().as_ref()) {
                files.push(path);
            }
        }
    }

    files
}

const PLATFORM: &str = {
    if !cfg!(target_os = "windows") {
        "unix"
    } else {
        "windows"
    }
};

fn main() {
    let cwd = current_dir().unwrap();
    let files = read_dir(cwd.join("src").join(PLATFORM));

    let mut builder = cc::Build::new();

    builder.files(files).compile(PLATFORM);
}

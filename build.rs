const PLATFORM: &str = {
    if cfg!(target_os = "windows") {
        "win"
    } else {
        "unix"
    }
};

fn main() {
    let file_path = format!("src/{}.c", PLATFORM);

    cc::Build::new().file(file_path).compile("admin");
}

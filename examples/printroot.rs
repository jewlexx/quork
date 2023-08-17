use quork::prelude::is_root;

fn main() {
    let root = is_root().unwrap();

    println!("Is the the process running as admin? {}", root);
}

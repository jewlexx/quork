use quork::prelude::is_root;

fn main() {
    let root = is_root();

    println!("Is the user an admin? {}", root);
}

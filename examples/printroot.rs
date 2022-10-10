use quork::is_root;

fn main() {
    let root = is_root();

    println!("Is the user an admin? {}", root);
}

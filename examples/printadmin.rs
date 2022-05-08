use quork::admin::is_admin;

fn main() {
    let admin = is_admin();

    println!("Is the user an admin? {}", admin);
}

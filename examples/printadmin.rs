fn main() {
    use is_admin::is_admin;

    let admin = is_admin();

    println!("Is the user an admin? {}", admin);
}

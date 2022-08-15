pub fn get_connection() {
    let interfaces = nix::net::if_::if_nameindex().unwrap();

    for iface in &interfaces {
        println!(
            "Interface #{} is called {}",
            iface.index(),
            iface.name().to_string_lossy()
        );
    }
}

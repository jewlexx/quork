use nix::ifaddrs::getifaddrs;

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub connected: bool,
}

pub fn get_connection() -> Vec<Interface> {
    let addrs = getifaddrs().unwrap();

    addrs
        .map(|addr| Interface {
            name: addr.interface_name,
            connected: addr.address.is_some(),
        })
        .collect()
}

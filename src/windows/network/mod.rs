use windows_api::Networking::Connectivity::NetworkUsage;

extern "C" {
    fn IsDisconnected() -> u8;
}

pub fn has_connection() -> bool {
    unsafe { IsDisconnected() != 1 }
}

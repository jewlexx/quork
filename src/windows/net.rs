// Move net.cc to Rust

#[repr(u8)]
pub enum InternetStatus {
    Connected,
    Disconnected,
    ConnectedToLocal,
    ConnectionError,
}

pub fn get_status() {
    let mut status = InternetStatus::ConnectionError;
}

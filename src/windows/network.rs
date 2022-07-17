#[cxx::bridge]
mod ffi {
    enum NetworkStatus {
        CONNECTED,
        DISCONNECTED,
        CONNECTED_TO_LOCAL,
        CONNECTION_ERROR,
    }

    unsafe extern "C++" {
        include!("../../include/check-net.hpp");
        type NetworkStatus;

        fn IsConnectedToNetwork() -> NetworkStatus;
    }
}

pub use ffi::NetworkStatus;

pub fn network_status() -> NetworkStatus {
    ffi::IsConnectedToNetwork()
}

pub fn is_connected_to_network() -> bool {
    let status = network_status();

    matches!(
        status,
        NetworkStatus::CONNECTED | NetworkStatus::CONNECTED_TO_LOCAL
    )
}

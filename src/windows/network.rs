#[cxx::bridge]
mod ffi {
    enum NetworkStatus {
        CONNECTED,
        DISCONNECTED,
        CONNECTED_TO_LOCAL,
        CONNECTION_ERROR,
    }

    unsafe extern "C++" {
        type NetworkStatus;

        fn IsConnectedToNetwork() -> NetworkStatus;
    }
}

pub use ffi::NetworkStatus;

/// Network Status
///
/// NOTE: This function **DOES NOT** detect if the user has the ability to access the internet. It only checks if they are connected to a network, which may not be connected to the internet
///
/// Will return the status of connection as [`NetworkStatus`]
pub fn network_status() -> NetworkStatus {
    ffi::IsConnectedToNetwork()
}

/// Network Detection
///
/// NOTE: This function **DOES NOT** detect if the user has the ability to access the internet. It only checks if they are connected to a network, which may not be connected to the internet
///
/// # Example
///
/// ```
/// # use quork::network::is_connected_to_network;
/// # fn main() {
/// let connected = is_connected_to_network();
///
/// if connected {
///     println!("We are connected to the internet");
/// } else {
///     println!("We are not connected to the internet");
/// }
/// # }
/// ```
pub fn is_connected_to_network() -> bool {
    let status = network_status();

    matches!(
        status,
        NetworkStatus::CONNECTED | NetworkStatus::CONNECTED_TO_LOCAL
    )
}

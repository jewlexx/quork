//! Network helpers

use windows::{
    core::GUID,
    Win32::{
        Networking::NetworkListManager::{INetworkListManager, NLM_CONNECTIVITY},
        System::Com::{CoCreateInstance, CLSCTX_ALL},
    },
};

use windows::Win32::Networking::NetworkListManager::{
    NLM_CONNECTIVITY_DISCONNECTED, NLM_CONNECTIVITY_IPV4_INTERNET,
    NLM_CONNECTIVITY_IPV4_LOCALNETWORK, NLM_CONNECTIVITY_IPV4_NOTRAFFIC,
    NLM_CONNECTIVITY_IPV4_SUBNET, NLM_CONNECTIVITY_IPV6_INTERNET,
    NLM_CONNECTIVITY_IPV6_LOCALNETWORK, NLM_CONNECTIVITY_IPV6_NOTRAFFIC,
    NLM_CONNECTIVITY_IPV6_SUBNET,
};

use super::ComInit;

/// A set of flags that give more information about the underlying connectivity to a network
pub enum Connectivity {
    /// The underlying network interfaces have no connectivity to any network.
    Disconnected = 0,
    /// There is connectivity to a network, but the service cannot detect any IPv4 Network Traffic.
    Ipv4Notraffic = 0x1,
    /// There is connectivity to a network, but the service cannot detect any IPv6 Network Traffic.
    Ipv6Notraffic = 0x2,
    /// There is connectivity to the local subnet using the IPv4 protocol.
    Ipv4Subnet = 0x10,
    /// There is connectivity to a routed network using the IPv4 protocol.
    Ipv4Localnetwork = 0x20,
    /// There is connectivity to the Internet using the IPv4 protocol.
    Ipv4Internet = 0x40,
    /// There is connectivity to the local subnet using the IPv6 protocol.
    Ipv6Subnet = 0x100,
    /// There is connectivity to a local network using the IPv6 protocol.
    Ipv6Localnetwork = 0x200,
    /// There is connectivity to the Internet using the IPv6 protocol.
    Ipv6Internet = 0x400,
}

impl Connectivity {
    /// Gets the current connectivity to a network.
    pub fn get() -> Self {
        Self::try_get().unwrap()
    }

    /// Tries to get the current connectivity to a network.
    ///
    /// # Errors
    /// - Can fail for any of the many reasons the internal windows API could fail
    /// - Can fail if the network result is invalid
    pub fn try_get() -> windows::core::Result<Self> {
        unsafe {
            let manager = get_networklist_manager()?;
            get_connectivity(&manager)
        }
    }
}

impl From<NLM_CONNECTIVITY> for Connectivity {
    fn from(connectivity: NLM_CONNECTIVITY) -> Self {
        match connectivity {
            NLM_CONNECTIVITY_DISCONNECTED => Connectivity::Disconnected,
            NLM_CONNECTIVITY_IPV4_INTERNET => Connectivity::Ipv4Internet,
            NLM_CONNECTIVITY_IPV4_LOCALNETWORK => Connectivity::Ipv4Localnetwork,
            NLM_CONNECTIVITY_IPV4_NOTRAFFIC => Connectivity::Ipv4Notraffic,
            NLM_CONNECTIVITY_IPV4_SUBNET => Connectivity::Ipv4Subnet,
            NLM_CONNECTIVITY_IPV6_INTERNET => Connectivity::Ipv6Internet,
            NLM_CONNECTIVITY_IPV6_LOCALNETWORK => Connectivity::Ipv6Localnetwork,
            NLM_CONNECTIVITY_IPV6_NOTRAFFIC => Connectivity::Ipv6Notraffic,
            NLM_CONNECTIVITY_IPV6_SUBNET => Connectivity::Ipv6Subnet,
            _ => Connectivity::Ipv4Notraffic,
        }
    }
}

impl From<Connectivity> for NLM_CONNECTIVITY {
    fn from(connectivity: Connectivity) -> Self {
        match connectivity {
            Connectivity::Disconnected => NLM_CONNECTIVITY_DISCONNECTED,
            Connectivity::Ipv4Internet => NLM_CONNECTIVITY_IPV4_INTERNET,
            Connectivity::Ipv4Localnetwork => NLM_CONNECTIVITY_IPV4_LOCALNETWORK,
            Connectivity::Ipv4Notraffic => NLM_CONNECTIVITY_IPV4_NOTRAFFIC,
            Connectivity::Ipv4Subnet => NLM_CONNECTIVITY_IPV4_SUBNET,
            Connectivity::Ipv6Internet => NLM_CONNECTIVITY_IPV6_INTERNET,
            Connectivity::Ipv6Localnetwork => NLM_CONNECTIVITY_IPV6_LOCALNETWORK,
            Connectivity::Ipv6Notraffic => NLM_CONNECTIVITY_IPV6_NOTRAFFIC,
            Connectivity::Ipv6Subnet => NLM_CONNECTIVITY_IPV6_SUBNET,
        }
    }
}

extern "C" {
    fn get_networklist_manager_clsid() -> GUID;
}

/// Gets the INetworkListManager COM interface class GUID.
///
/// Not reccomended for use directly, but rather though the [`Connectivity`] enum
///
/// # Safety
/// - Interacts with windows api
pub unsafe fn get_networklist_manager() -> windows::core::Result<INetworkListManager> {
    ComInit::init();

    CoCreateInstance(&get_networklist_manager_clsid(), None, CLSCTX_ALL)
}

/// Gets the current connectivity to a network.
///
/// Not reccomended for use directly, but rather though the [`Connectivity`] enum
///
/// # Safety
/// - Interacts with windows api
pub unsafe fn get_connectivity(
    manager: &INetworkListManager,
) -> windows::core::Result<Connectivity> {
    let conn = manager.GetConnectivity()?;
    Ok(conn.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manager() {
        assert_eq!(unsafe { get_networklist_manager() }.err(), None);
    }
}

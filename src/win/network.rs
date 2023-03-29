//! Network helpers

use windows::Win32::{
    Networking::NetworkListManager::INetworkListManager, System::Com::CoCreateInstance,
};

pub unsafe fn get_manager() {
    let layout = {
        let size = std::mem::size_of::<INetworkListManager>();

        std::mem::transmute::<_, *mut INetworkListManager>(std::mem::zeroed::<usize>())
    };

    CoCreateInstance(CLSID_NetworkListManager, punkouter, dwclscontext);
}

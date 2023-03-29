//! Network helpers

use windows::Win32::{
    Networking::NetworkListManager::INetworkListManager, System::Com::CoCreateInstance,
};

extern "C" {
    static CLSID_NetworkListManager: *const windows::core::GUID;
}

pub unsafe fn get_manager() {
    println!("clsid at: {:?}", *CLSID_NetworkListManager);
    // let layout = {
    //     let size = std::mem::size_of::<INetworkListManager>();

    //     std::mem::transmute::<_, *mut INetworkListManager>(std::mem::zeroed::<usize>())
    // };

    // CoCreateInstance(CLSID_NetworkListManager, punkouter, dwclscontext);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manager() {
        unsafe { get_manager() };
    }
}

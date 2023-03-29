//! Network helpers

use windows::{
    core::GUID,
    Win32::{Networking::NetworkListManager::INetworkListManager, System::Com::CoCreateInstance},
};

extern "C" {
    fn get_networklist_manager_clsid() -> GUID;
}

pub unsafe fn get_manager() {
    let layout = {
        let size = std::mem::size_of::<INetworkListManager>();

        std::mem::transmute::<_, *mut INetworkListManager>(std::mem::zeroed::<usize>())
    };

    CoCreateInstance(
        &get_networklist_manager_clsid(),
        std::mem::zeroed(),
        dwclscontext,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manager() {
        unsafe { get_manager() };
    }
}

//! Network helpers

use windows::{
    core::GUID,
    Win32::{
        Networking::NetworkListManager::INetworkListManager,
        System::Com::{CoCreateInstance, CLSCTX_ALL},
    },
};

use super::ComInit;

extern "C" {
    fn get_networklist_manager_clsid() -> GUID;
}

pub unsafe fn get_manager() -> windows::core::Result<()> {
    ComInit::init();

    let layout = {
        let size = std::mem::size_of::<INetworkListManager>();

        std::mem::transmute::<_, *mut INetworkListManager>(std::mem::zeroed::<usize>())
    };

    let manager: INetworkListManager =
        CoCreateInstance(&get_networklist_manager_clsid(), None, CLSCTX_ALL)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manager() {
        assert_eq!(unsafe { get_manager() }.err(), None);
    }
}

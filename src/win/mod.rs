use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

pub struct ComInit {
    initialized: bool,
}

impl ComInit {
    pub fn init() -> Self {
        Self {
            initialized: unsafe { Self::init_com() }.is_ok(),
        }
    }

    unsafe fn init_com() -> Result<(), windows::core::Error> {
        CoInitializeEx(None, COINIT_MULTITHREADED)
    }
}

impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

pub mod network;

use std::sync::Mutex;

use once_cell::sync::Lazy;

use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

pub(crate) static COM_INIT: Mutex<Lazy<ComInit>> = Mutex::new(Lazy::new(ComInit::init));

pub(crate) struct ComInit {
    initialized: bool,
}

impl ComInit {
    pub fn init() -> Self {
        Self {
            initialized: unsafe { Self::init_com() }.is_ok(),
        }
    }

    unsafe fn init_com() -> Result<(), windows::core::Error> {
        if COM_INIT.try_lock().map(|lock| lock.initialized) {}

        CoInitializeEx(None, COINIT_MULTITHREADED)
    }
}

impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

pub mod network;

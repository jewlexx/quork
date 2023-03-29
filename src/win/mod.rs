use std::sync::{atomic::AtomicBool, Mutex};

use once_cell::sync::Lazy;

use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

use crate::IsTrue;

pub(crate) static COM_INIT: AtomicBool = AtomicBool::new(false);

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
        if COM_INIT {
            Ok(())
        } else {
            CoInitializeEx(None, COINIT_MULTITHREADED)
        }
    }
}

impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

pub mod network;

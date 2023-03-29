use std::sync::atomic::{AtomicBool, Ordering};

use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

use crate::prelude::FlipImmut;

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
        if COM_INIT.flipped() {
            CoInitializeEx(None, COINIT_MULTITHREADED)?;
            COM_INIT.store(true, Ordering::Relaxed);
        }
        Ok(())
    }
}

impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

pub mod network;

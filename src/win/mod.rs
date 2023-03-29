use std::sync::{atomic::Ordering, Mutex};

use once_cell::sync::Lazy;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

use crate::IsTrue;

pub(crate) static COM_INIT: Mutex<Lazy<ComInit>> = Mutex::new(Lazy::new(ComInit::init));

#[derive(Debug, Clone)]
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
        let mut com_init = COM_INIT.try_lock().unwrap();
        if !com_init.initialized {
            CoInitializeEx(None, COINIT_MULTITHREADED)?;
            com_init.initialized = true;
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

//! Windows specific functionality.

use std::sync::Mutex;

use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "root")]
pub mod root;

pub(crate) static COM_INIT: Mutex<ComInit> = Mutex::new(ComInit { initialized: false });

#[derive(Debug, Clone)]
pub(crate) struct ComInit {
    initialized: bool,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ComError {
    #[error("Failed to lock COM_INIT variable: {0}")]
    LockError(#[from] std::sync::PoisonError<std::sync::MutexGuard<'static, ComInit>>),
}

impl ComInit {
    pub(crate) unsafe fn init() {
        let mut com_init = COM_INIT.lock().expect("unpoisoned mutex");
        if !com_init.initialized {
            com_init.initialized = CoInitializeEx(None, COINIT_MULTITHREADED).is_ok();
        }
    }
}

// As COM_INIT is a static variable this will be dropped at the end of the program.
impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

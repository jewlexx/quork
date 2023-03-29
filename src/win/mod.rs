use std::sync::Mutex;

use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

pub(crate) static COM_INIT: Mutex<ComInit> = Mutex::new(ComInit { initialized: false });

#[derive(Debug, Clone)]
pub(crate) struct ComInit {
    initialized: bool,
}

impl ComInit {
    pub fn init() {
        let is_init = unsafe { Self::init_com() }.is_ok();
        COM_INIT.try_lock().unwrap().initialized = is_init;
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

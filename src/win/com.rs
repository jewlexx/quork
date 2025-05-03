#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

use windows_sys::Win32::{
    Foundation::S_OK,
    System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED},
};

pub(crate) static COM_INIT: ComInit = ComInit {
    initialized: AtomicBool::new(false),
};

#[derive(Debug)]
pub(crate) struct ComInit {
    initialized: AtomicBool,
}

impl ComInit {
    pub(crate) unsafe fn init() {
        if !COM_INIT.initialized.load(Ordering::Relaxed) {
            COM_INIT.initialized.store(
                CoInitializeEx(core::ptr::null(), COINIT_MULTITHREADED as u32) == S_OK,
                Ordering::Relaxed,
            );
        }
    }
}

// As COM_INIT is a static variable this will be dropped at the end of the program.
impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

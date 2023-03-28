//! Network helpers

#[cfg(windows)]
mod windows {
    use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

    pub unsafe fn init_com() -> Result<(), windows::core::Error> {
        CoInitializeEx(None, COINIT_MULTITHREADED)
    }

    pub unsafe fn uninit_com() {
        CoUninitialize()
    }
}

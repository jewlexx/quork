/// Import desired modules for target
#[macro_export]
macro_rules! import_ffi {
    () => {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "windows")] {
                use $crate::windows as ffi;
            } else {
                use $crate::unix as ffi;
            }
        }
    };
}

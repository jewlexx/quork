/// Import desired modules for target
#[macro_export]
macro_rules! import_ffi {
    () => {
        #[cfg(not(target_os = "windows"))]
        use $crate::unix as ffi;
        #[cfg(target_os = "windows")]
        use $crate::windows as ffi;
    };
}

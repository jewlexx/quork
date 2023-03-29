cfg_if::cfg_if! {
    if #[cfg(windows)] {
        use crate::win::root::is_elevated as is_root;
    }
}

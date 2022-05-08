#![warn(missing_docs)]

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub mod windows;
        pub use windows::*;
    } else {
        pub mod unix;
        pub use unix::*;
    }
}

#![warn(missing_docs)]

pub use quork_proc as macros;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub mod windows;
        pub use windows::*;
    } else {
        pub mod unix;
        pub use unix::*;
    }
}

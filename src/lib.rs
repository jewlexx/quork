#![warn(missing_docs)]

//! A little crate that helps with interacting with the system
//!
//! The lower levels can be hard. So this crate is intended to help with that by abstracing the lower levels to high level code

pub use quork_proc as macros;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else {
        mod unix;
    }
}

pub mod admin;

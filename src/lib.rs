#![warn(missing_docs)]

//! A little crate that helps with interacting with the system
//!
//! The lower levels can be hard. So this crate is intended to help with that by abstracing the lower levels to high level code

pub mod prelude {
    //! `use quork::prelude::*` To include common helpful items

    cfg_if::cfg_if! {
        if #[cfg(feature = "traits")] {
            pub use crate::traits::flip::*;
            pub use crate::traits::lock::*;
            pub use crate::traits::truthy::*;
        }
    }

    #[cfg(feature = "macros")]
    pub use crate::macros::*;

    #[cfg(feature = "root")]
    pub use crate::root::is_root;
}

#[cfg(windows)]
pub mod win;

#[cfg(unix)]
pub mod unix;

#[cfg(feature = "macros")]
pub use quork_proc as macros;

#[cfg(feature = "traits")]
pub mod traits;

#[cfg(feature = "network")]
pub mod network;

cfg_if::cfg_if! {
    if #[cfg(feature = "root")] {
        pub mod root;
    }
}

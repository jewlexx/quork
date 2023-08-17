#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod prelude {
    //! `use quork::prelude::*` To include common helpful items

    cfg_if::cfg_if! {
        if #[cfg(feature = "traits")] {
            pub use crate::traits::flip::*;
            pub use crate::traits::lock::*;
            pub use crate::traits::truthy::*;
            pub use crate::traits::list::*;
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

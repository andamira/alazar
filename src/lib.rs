// alazar::lib
//
//! Random number generation.
//

// warnings
#![warn(clippy::all)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");
// deprecated
devela::deprecate_feature![old: "all", new: "full", since: "0.0.2"];

pub mod misc;
pub mod xorshift;

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    pub use super::{misc::*, xorshift::*};
}

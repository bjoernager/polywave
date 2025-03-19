// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

//! Vector-based colour manipulators.

#![warn(missing_docs)]

#![no_std]

#![cfg_attr(feature = "f16",  feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

extern crate self as polywave;

#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
extern crate alloc;

pub mod colour;
pub mod error;

mod component;

pub use component::Component;

use component::SealedComponent;

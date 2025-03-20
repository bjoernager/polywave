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

pub mod error;
pub mod hsv;
pub mod lab;
pub mod rgb;
pub mod www;
pub mod xyz;

mod colour;
mod component;
mod defined_gamut;

pub use colour::Colour;
pub use component::Component;
pub use defined_gamut::DefinedGamut;

use component::SealedComponent;

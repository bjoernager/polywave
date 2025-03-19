// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

//! The [`Colour`] trait and colour types.

mod colour;
mod css;
mod rgba;

pub use colour::Colour;
pub use css::Css;
pub use rgba::Rgba;

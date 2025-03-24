// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{BalancedColour, Colour, Component};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zerocopy")]
use zerocopy::{FromZeros, Immutable, IntoBytes};

/// An HWB colour.
///
/// This type guarantees that its three channels -- hue, whiteness, and blackness -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable, IntoBytes))]
pub struct Hwb<T>([T; 0x3]);

impl<T: Component> Hwb<T> {
	/// Constructs a new HWB colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(hue: T, whiteness: T, blackness: T) -> Self {
		let data = [hue, whiteness, blackness];
		Self(data)
	}

	/// Deconstructs an HWB colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [hue, whiteness, blackness] = self.0;
		(hue, whiteness, blackness)
	}
}

unsafe impl<T: Component> BalancedColour for Hwb<T> {
	type Component = T;
}

impl<T: Component> Colour for Hwb<T> { }

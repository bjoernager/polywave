// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component, DefinedGamut};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zerocopy")]
use zerocopy::{FromZeros, Immutable, IntoBytes};

/// A CIEXYZ colour.
///
/// This type guarantees that its three channels -- X, Y, and Z -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable, IntoBytes))]
pub struct CieXyz<T>([T; 0x3]);

impl<T: Component> CieXyz<T> {
	/// Constructs a new CIEXYZ colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(x: T, y: T, z: T) -> Self {
		let data = [x, y, z];
		Self(data)
	}

	/// Deconstructs a CIEXYZ colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [x, y, z] = self.0;
		(x, y, z)
	}
}

impl<T: Component> Colour for CieXyz<T> { }

impl<T: Component> DefinedGamut for CieXyz<T> { }

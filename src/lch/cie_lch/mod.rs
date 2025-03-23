// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component, DefinedGamut};
use crate::lch::CieLcha;

/// A CIELCh colour.
///
/// This type guarantees that its three channels -- luminance, chroma, and hue -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct CieLch<T>([T; 0x3]);

impl<T: Component> CieLch<T> {
	/// Constructs a new CIELCh colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(x: T, y: T, z: T) -> Self {
		let data = [x, y, z];
		Self(data)
	}

	/// Adds an alpha channel to the CIELCh colour.
	#[inline(always)]
	#[must_use]
	pub const fn with_alpha(self, alpha: T) -> CieLcha<T> {
		let (x, y, z) = self.get();
		CieLcha::new(x, y, z, alpha)
	}

	/// Deconstructs a CIELCh colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [x, y, z] = self.0;
		(x, y, z)
	}
}

impl<T: Component> Colour for CieLch<T> { }

impl<T: Component> DefinedGamut for CieLch<T> { }

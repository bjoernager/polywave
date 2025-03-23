// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component};
use crate::hsv::Hwba;

/// An HWB colour.
///
/// This type guarantees that its three channels -- hue, whiteness, and blackness -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct Hwb<T>([T; 0x3]);

impl<T: Component> Hwb<T> {
	/// Constructs a new HWB colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(hue: T, whiteness: T, blackness: T) -> Self {
		let data = [hue, whiteness, blackness];
		Self(data)
	}

	/// Adds an alpha channel to the HWB colour.
	#[inline(always)]
	#[must_use]
	pub const fn with_alpha(self, alpha: T) -> Hwba<T> {
		let (hue, whiteness, blackness) = self.get();
		Hwba::new(hue, whiteness, blackness, alpha)
	}

	/// Deconstructs an HWB colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [hue, whiteness, blackness] = self.0;
		(hue, whiteness, blackness)
	}
}

impl<T: Component> Colour for Hwb<T> { }

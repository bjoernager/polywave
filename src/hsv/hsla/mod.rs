// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component};
use crate::hsv::Hsv;

/// An HSLA colour.
///
/// This type guarantees that its four channels -- hue, saturation, luminosity (or *lightness*), and alpha -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct Hsla<T>([T; 0x4]);

impl<T: Component> Hsla<T> {
	/// Constructs a new HSLA colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(hue: T, saturation: T, luminosity: T, alpha: T) -> Self {
		let data = [hue, saturation, luminosity, alpha];
		Self(data)
	}

	/// Discards the HSLA colour's alpha channel.
	#[inline(always)]
	#[must_use]
	pub const fn discard_alpha(self) -> (Hsv<T>, T) {
		let (hue, saturation, luminosity, alpha) = self.get();

		let colour = Hsv::new(hue, saturation, luminosity);
		(colour, alpha)
	}

	/// Deconstructs an HSLA colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T, T) {
		let [hue, saturation, luminosity, alpha] = self.0;
		(hue, saturation, luminosity, alpha)
	}
}

impl<T: Component> Colour for Hsla<T> { }

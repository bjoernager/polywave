// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component};
use crate::hsv::Hsla;

/// An HSL colour.
///
/// This type guarantees that its three channels -- hue, saturation, and luminosity (or *lightness*) -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct Hsl<T>([T; 0x3]);

impl<T: Component> Hsl<T> {
	/// Constructs a new HSL colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(hue: T, saturation: T, luminosity: T) -> Self {
		let data = [hue, saturation, luminosity];
		Self(data)
	}

	/// Adds an alpha channel to the HSL colour.
	#[inline(always)]
	#[must_use]
	pub const fn with_alpha(self, alpha: T) -> Hsla<T> {
		let (hue, saturation, luminosity) = self.get();
		Hsla::new(hue, saturation, luminosity, alpha)
	}

	/// Deconstructs an HSL colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [hue, saturation, luminosity] = self.0;
		(hue, saturation, luminosity)
	}
}

impl<T: Component> Colour for Hsl<T> { }

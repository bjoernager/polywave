// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod from_str;
mod named;
mod test;

use crate::colour::{Colour, Rgba};

use core::fmt::{self, Debug, Display, Formatter};

/// A CSS colour.
///
/// This type is guaranteed to have the exact same layout and representation as <code>[Rgba]&lt;[u8]&gt;</code> in permanent [`sRGB`](https://en.wikipedia.org/wiki/SRGB), and -- for all intends and purposes -- these types are also equivalent.
///
/// Where this type differs is in that it may be more suited in user interactions due to its [`FromStr`](core::str::FromStr) and [`Display`] implementations.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct Css(Rgba<u8>);

impl Css {
	/// Constructs a new CSS colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		let colour = Rgba::new(red, green, blue, alpha);
		Self::from_rgba(colour)
	}

	/// Constructs a new CSS colour from [`u32`].
	///
	/// The `u32` value is reinterpreted as a four contiguous `u8` objects corresponding to each of the four channels.
	#[inline(always)]
	#[must_use]
	pub const fn from_u32(value: u32) -> Self {
		let [red, green, blue, alpha] = value.to_be_bytes();
		Self::new(red, green, blue, alpha)
	}

	/// Converts an SRGBA colour into a CSS colour.
	///
	/// This conversion is always lossless and zero-cost.
	#[inline(always)]
	#[must_use]
	pub const fn from_rgba(colour: Rgba<u8>) -> Self {
		Self(colour)
	}

	/// Deconstructs a CSS colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (u8, u8, u8, u8) {
		self.to_rgba().get()
	}

	/// Converts a CSS colour to [`u32`].
	///
	/// This function is the inverse of [`from_u32`](Self::from_u32) (see there for more information).
	#[inline(always)]
	#[must_use]
	pub const fn to_u32(self) -> u32 {
		let (r, g, b, a) = self.get();

		let data = [r, g, b, a];
		u32::from_be_bytes(data)
	}

	/// Converts a CSS colour to [`Rgba<u8>`].
	#[inline(always)]
	#[must_use]
	pub const fn to_rgba(self) -> Rgba<u8> {
		self.0
	}
}

impl Colour for Css {
	#[cfg(feature = "wgpu")]
	fn to_wgpu_color_lossy(&self) -> wgpu_types::Color {
		self.to_rgba().to_wgpu_color_lossy()
	}
}

impl Display for Css {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let value = self.to_u32();

		write!(f, "#{value:08X}")
	}
}

impl From<(u8, u8, u8, u8)> for Css {
	#[inline(always)]
	fn from((red, green, blue, alpha): (u8, u8, u8, u8)) -> Self {
		Self::new(red, green, blue, alpha)
	}
}

impl From<u32> for Css {
	#[inline(always)]
	fn from(value: u32) -> Self {
		Self::from_u32(value)
	}
}

impl From<Rgba<u8>> for Css {
	#[inline(always)]
	fn from(value: Rgba<u8>) -> Self {
		Self(value)
	}
}

impl From<Css> for u32 {
	#[inline(always)]
	fn from(value: Css) -> Self {
		value.to_u32()
	}
}

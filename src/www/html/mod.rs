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

use crate::{Alpha, BalancedColour, Colour, DefinedGamut};
use crate::rgb::SRgb;

use core::fmt::{self, Debug, Display, Formatter};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zerocopy")]
use zerocopy::{FromZeros, Immutable, IntoBytes};

/// An HTML colour.
///
/// This type is guaranteed to have the exact same layout and representation as <code>[Alpha]&lt;[SRgb]&lt;[u8]&gt;&gt;</code>, and -- for all intends and purposes -- these types are also equivalent.
///
/// Where this type differs is in that it may be more suited in user interactions due to its [`FromStr`](core::str::FromStr) and [`Display`] implementations.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable, IntoBytes))]
pub struct Html(Alpha<SRgb<u8>>);

impl Html {
	/// Constructs a new HTML colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		let colour = SRgb::new(red, green, blue);
		let colour = Alpha::attach(colour, alpha);

		Self::from_s_rgba(colour)
	}

	/// Constructs a new HTML colour from [`u32`].
	///
	/// The `u32` value is reinterpreted as a four contiguous `u8` objects corresponding to each of the four channels.
	#[inline(always)]
	#[must_use]
	pub const fn from_u32(value: u32) -> Self {
		let [red, green, blue, alpha] = value.to_be_bytes();
		Self::new(red, green, blue, alpha)
	}

	/// Converts an SRGBA colour into an HTML colour.
	///
	/// This conversion is always lossless and zero-cost.
	#[inline(always)]
	#[must_use]
	pub const fn from_s_rgba(colour: Alpha<SRgb<u8>>) -> Self {
		Self(colour)
	}

	/// Deconstructs an HTML colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (u8, u8, u8, u8) {
		let (colour, alpha) = self.to_s_rgba().detach();

		let (red, green, blue) = colour.get();

		(red, green, blue, alpha)
	}

	/// Converts an HTML colour to [`u32`].
	///
	/// This function is the inverse of [`from_u32`](Self::from_u32) (see there for more information).
	#[inline(always)]
	#[must_use]
	pub const fn to_u32(self) -> u32 {
		let (r, g, b, a) = self.get();

		let data = [r, g, b, a];
		u32::from_be_bytes(data)
	}

	/// Converts an HTML colour to [`Alpha<SRgb<u8>>`].
	///
	/// This conversion is always lossless and zero-cost.
	#[inline(always)]
	#[must_use]
	pub const fn to_s_rgba(self) -> Alpha<SRgb<u8>> {
		self.0
	}
}

unsafe impl BalancedColour for Html {
	type Component = u8;
}

impl Colour for Html { }

impl DefinedGamut for Html { }

impl Display for Html {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let value = self.to_u32();

		write!(f, "#{value:08X}")
	}
}

impl From<u32> for Html {
	#[inline(always)]
	fn from(value: u32) -> Self {
		Self::from_u32(value)
	}
}

impl From<Alpha<SRgb<u8>>> for Html {
	#[inline(always)]
	fn from(value: Alpha<SRgb<u8>>) -> Self {
		Self(value)
	}
}

impl From<SRgb<u8>> for Html {
	#[inline(always)]
	fn from(value: SRgb<u8>) -> Self {
		let colour = Alpha::attach(value, 0xFF);
		Self(colour)
	}
}

impl From<Html> for Alpha<SRgb<u8>> {
	#[inline(always)]
	fn from(value: Html) -> Self {
		value.to_s_rgba()
	}
}

impl From<Html> for u32 {
	#[inline(always)]
	fn from(value: Html) -> Self {
		value.to_u32()
	}
}

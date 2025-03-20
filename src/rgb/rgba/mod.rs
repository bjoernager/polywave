// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component};
use crate::rgb::Rgb;

/// A raw RGBA colour.
///
/// This type guarantees that its four channels -- red, green, blue, and alpha -- are stored sequentially in memory (in this order).
///
/// Unlike other colours such as [SRgba](crate::rgb::SRgba), this one does not define a gamut *per se*; instead, users of this type are to interpret it on their own.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct Rgba<T>([T; 0x4]);

impl<T: Component> Rgba<T> {
	/// Constructs a new RGBA colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(red: T, green: T, blue: T, alpha: T) -> Self {
		let data = [red, green, blue, alpha];
		Self(data)
	}

	/// Discards the RGBA colour's alpha channel.
	#[inline(always)]
	#[must_use]
	pub const fn discard_alpha(self) -> Rgb<T> {
		let (red, green, blue, _) = self.get();
		Rgb::new(red, green, blue)
	}

	/// Deconstructs an RGBA colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T, T) {
		let [red, green, blue, alpha] = self.0;
		(red, green, blue, alpha)
	}
}

impl<T: Component> Colour for Rgba<T> { }

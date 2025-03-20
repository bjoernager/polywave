// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component, DefinedGamut};
use crate::rgb::{OpRgba, Rgb};

/// An opRGB colour.
///
/// This type is guaranteed to always have the exact same layout as <code>[Rgb]&lt;T&gt;</code>, although with a different representation.
/// Namely, channel order is preserved in memory.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct OpRgb<T>(Rgb<T>);

impl<T: Component> OpRgb<T> {
	/// Constructs a new opRGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(red: T, green: T, blue: T) -> Self {
		let colour = Rgb::new(red, green, blue);
		Self(colour)
	}

	/// Adds an alpha channel to the opRGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn with_alpha(self, alpha: T) -> OpRgba<T> {
		let (red, green, blue) = self.get();
		OpRgba::new(red, green, blue, alpha)
	}

	/// Reinterprets the opRGB colour as a raw RGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn as_rgb(self) -> Rgb<T> {
		self.0
	}

	/// Deconstructs an opRGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		self.as_rgb().get()
	}
}

impl<T: Component> Colour for OpRgb<T> { }

impl<T: Component> DefinedGamut for OpRgb<T> { }

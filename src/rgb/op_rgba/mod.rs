// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component, DefinedGamut};
use crate::rgb::{OpRgb, Rgba};

/// An opRGBA colour.
///
/// This type is guaranteed to always have the exact same layout as <code>[Rgba]&lt;T&gt;</code>, although with a different representation.
/// Namely, channel order is preserved in memory.
///
/// This type is equivalent to [`OpRgb`] with an added alpha component.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct OpRgba<T>(Rgba<T>);

impl<T: Component> OpRgba<T> {
	/// Constructs a new opRGBA colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(red: T, green: T, blue: T, alpha: T) -> Self {
		let colour = Rgba::new(red, green, blue, alpha);
		Self(colour)
	}

	/// Discards the opRGBA colour's alpha channel.
	#[inline(always)]
	#[must_use]
	pub const fn discard_alpha(self) -> OpRgb<T> {
		let (red, green, blue, _) = self.get();
		OpRgb::new(red, green, blue)
	}

	/// Reinterprets the opRGBA colour as a raw RGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn as_rgba(self) -> Rgba<T> {
		self.0
	}

	/// Deconstructs an RGBA colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T, T) {
		self.as_rgba().get()
	}
}

impl<T: Component> Colour for OpRgba<T> { }

impl<T: Component> DefinedGamut for OpRgba<T> { }

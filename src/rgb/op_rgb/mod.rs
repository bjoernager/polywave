// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{BalancedColour, Colour, Component, DefinedGamut};
use crate::rgb::Rgb;

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zerocopy")]
use zerocopy::{FromZeros, Immutable, IntoBytes};

/// An opRGB (also known as Adobe RGB) colour.
///
/// This type is guaranteed to always have the exact same layout as <code>[Rgb]&lt;T&gt;</code>, although with a different representation.
/// Namely, channel order is preserved in memory.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable, IntoBytes))]
pub struct OpRgb<T>(Rgb<T>);

impl<T: Component> OpRgb<T> {
	/// Constructs a new opRGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(red: T, green: T, blue: T) -> Self {
		let colour = Rgb::new(red, green, blue);
		Self(colour)
	}

	/// Maps the opRGB colour's channels.
	#[inline]
	#[must_use]
	pub fn map<U, F>(self, mut op: F) -> OpRgb<U>
	where
		U: Component,
		F: FnMut(T) -> U,
	{
		let (red, green, blue) = self.get();

		let red   = op(red);
		let green = op(green);
		let blue  = op(blue);

		OpRgb::new(red, green, blue)
	}

	/// Reinterprets the opRGB colour as a raw RGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn as_rgb(self) -> Rgb<T> {
		self.0
	}

	/// Deconstructs the opRGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		self.as_rgb().get()
	}
}

unsafe impl<T: Component> BalancedColour for OpRgb<T> {
	type Component = T;
}

impl<T: Component> Colour for OpRgb<T> { }

impl<T: Component> DefinedGamut for OpRgb<T> { }

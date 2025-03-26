// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{BalancedColour, Colour, Component};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zerocopy")]
use zerocopy::{FromZeros, Immutable, IntoBytes};

/// An HSL colour.
///
/// This type guarantees that its three channels -- hue, saturation, and luminosity (or *lightness*) -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable, IntoBytes))]
pub struct Hsl<T>([T; 0x3]);

impl<T: Component> Hsl<T> {
	/// Constructs a new HSL colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(hue: T, saturation: T, luminosity: T) -> Self {
		let data = [hue, saturation, luminosity];
		Self(data)
	}

	/// Maps the HSL colour's channels.
	#[inline]
	#[must_use]
	pub fn map<U, F>(self, mut op: F) -> Hsl<U>
	where
		U: Component,
		F: FnMut(T) -> U,
	{
		let (hue, saturation, luminosity) = self.get();

		let hue        = op(hue);
		let saturation = op(saturation);
		let luminosity = op(luminosity);

		Hsl::new(hue, saturation, luminosity)
	}

	/// Deconstructs the HSL colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [hue, saturation, luminosity] = self.0;
		(hue, saturation, luminosity)
	}
}

unsafe impl<T: Component> BalancedColour for Hsl<T> {
	type Component = T;
}

impl<T: Component> Colour for Hsl<T> { }

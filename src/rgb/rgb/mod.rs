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

/// A raw RGB colour.
///
/// This type guarantees that its three channels -- red, green, and blue -- are stored sequentially in memory (in this order).
///
/// Unlike other colours such as [SRgb](crate::rgb::SRgb), this one does not define a gamut *per se*; instead, users of this type are to interpret it on their own.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable, IntoBytes))]
pub struct Rgb<T>([T; 0x3]);

impl<T: Component> Rgb<T> {
	/// Constructs a new, raw RGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(red: T, green: T, blue: T) -> Self {
		let data = [red, green, blue];
		Self(data)
	}

	/// Deconstructs a raw RGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [red, green, blue] = self.0;
		(red, green, blue)
	}
}

unsafe impl<T: Component> BalancedColour for Rgb<T> {
	type Component = T;
}

impl<T: Component> Colour for Rgb<T> { }

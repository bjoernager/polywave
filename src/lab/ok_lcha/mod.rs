// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component, DefinedGamut};
use crate::lab::OkLch;

/// An Oklcha colour.
///
/// This type guarantees that its three channels -- luminance, chroma, hue, and alpha -- are stored sequentially in memory (in this order).
///
/// This type is equivalent to [`OkLch`] with an added alpha component.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct OkLcha<T>([T; 0x4]);

impl<T: Component> OkLcha<T> {
	/// Constructs a new Oklcha colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(x: T, y: T, z: T, alpha: T) -> Self {
		let data = [x, y, z, alpha];
		Self(data)
	}

	/// Discards the Oklcha colour's alpha channel.
	#[inline(always)]
	#[must_use]
	pub const fn discard_alpha(self) -> OkLch<T> {
		let (x, y, z, _) = self.get();
		OkLch::new(x, y, z)
	}

	/// Deconstructs an Oklcha colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T, T) {
		let [x, y, z, alpha] = self.0;
		(x, y, z, alpha)
	}
}

impl<T: Component> Colour for OkLcha<T> { }

impl<T: Component> DefinedGamut for OkLcha<T> { }

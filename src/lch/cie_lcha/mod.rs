// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component, DefinedGamut};
use crate::lch::CieLch;

/// A CIELChA colour.
///
/// This type guarantees that its three channels -- luminance, chroma, hue, and alpha -- are stored sequentially in memory (in this order).
///
/// This type is equivalent to [`CieLch`] with an added alpha component.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct CieLcha<T>([T; 0x4]);

impl<T: Component> CieLcha<T> {
	/// Constructs a new CIELChA colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(lightness: T, chroma: T, hue: T, alpha: T) -> Self {
		let data = [lightness, chroma, hue, alpha];
		Self(data)
	}

	/// Discards the CIELChA colour's alpha channel.
	#[inline(always)]
	#[must_use]
	pub const fn discard_alpha(self) -> (CieLch<T>, T) {
		let (lightness, chroma, hue, alpha) = self.get();

		let colour = CieLch::new(lightness, chroma, hue);
		(colour, alpha)
	}

	/// Deconstructs a CIELChA colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T, T) {
		let [lightness, chroma, hue, alpha] = self.0;
		(lightness, chroma, hue, alpha)
	}
}

impl<T: Component> Colour for CieLcha<T> { }

impl<T: Component> DefinedGamut for CieLcha<T> { }

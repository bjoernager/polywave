// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component, DefinedGamut};
use crate::lab::OkLab;

/// An OkLaba colour.
///
/// This type guarantees that its three channels -- luminance, a*, b*, and alpha -- are stored sequentially in memory (in this order).
///
/// This type is equivalent to [`OkLab`] with an added alpha component.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct OkLaba<T>([T; 0x4]);

impl<T: Component> OkLaba<T> {
	/// Constructs a new Oklaba colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(lightness: T, a_star: T, b_star: T, alpha: T) -> Self {
		let data = [lightness, a_star, b_star, alpha];
		Self(data)
	}

	/// Discards the Oklaba colour's alpha channel.
	#[inline(always)]
	#[must_use]
	pub const fn discard_alpha(self) -> (OkLab<T>, T) {
		let (lightness, a_star, b_star, alpha) = self.get();

		let colour = OkLab::new(lightness, a_star, b_star);
		(colour, alpha)
	}

	/// Deconstructs an Oklaba colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T, T) {
		let [lightness, a_star, b_star, alpha] = self.0;
		(lightness, a_star, b_star, alpha)
	}
}

impl<T: Component> Colour for OkLaba<T> { }

impl<T: Component> DefinedGamut for OkLaba<T> { }

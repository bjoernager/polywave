// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{BalancedColour, Colour, Component, DefinedGamut};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zerocopy")]
use zerocopy::{FromZeros, Immutable, IntoBytes};

/// An Oklab colour.
///
/// This type guarantees that its three channels -- luminance, a*, and b* -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable, IntoBytes))]
pub struct OkLab<T>([T; 0x3]);

impl<T: Component> OkLab<T> {
	/// Constructs a new Oklab colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(luminance: T, a_star: T, b_star: T) -> Self {
		let data = [luminance, a_star, b_star];
		Self(data)
	}

	/// Maps the Oklab colour's channels.
	#[inline]
	#[must_use]
	pub fn map<U, F>(self, mut op: F) -> OkLab<U>
	where
		U: Component,
		F: FnMut(T) -> U,
	{
		let (luminance, a_star, b_star) = self.get();

		let luminance = op(luminance);
		let a_star    = op(a_star);
		let b_star    = op(b_star);

		OkLab::new(luminance, a_star, b_star)
	}

	/// Deconstructs the Oklab colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [luminance, a_star, b_star] = self.0;
		(luminance, a_star, b_star)
	}
}

unsafe impl<T: Component> BalancedColour for OkLab<T> {
	type Component = T;
}

impl<T: Component> Colour for OkLab<T> { }

impl<T: Component> DefinedGamut for OkLab<T> { }

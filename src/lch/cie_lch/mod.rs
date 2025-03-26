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

/// A CIELCh colour.
///
/// This type guarantees that its three channels -- luminance, chroma, and hue -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable, IntoBytes))]
pub struct CieLch<T>([T; 0x3]);

impl<T: Component> CieLch<T> {
	/// Constructs a new CIELCh colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(luminance: T, chroma: T, hue: T) -> Self {
		let data = [luminance, chroma, hue];
		Self(data)
	}

	/// Maps the CIELCh colour's channels.
	#[inline]
	#[must_use]
	pub fn map<U, F>(self, mut op: F) -> CieLch<U>
	where
		U: Component,
		F: FnMut(T) -> U,
	{
		let (luminance, chroma, hue) = self.get();

		let luminance = op(luminance);
		let chroma    = op(chroma);
		let hue       = op(hue);

		CieLch::new(luminance, chroma, hue)
	}

	/// Deconstructs the CIELCh colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [luminance, chroma, hue] = self.0;
		(luminance, chroma, hue)
	}
}

unsafe impl<T: Component> BalancedColour for CieLch<T> {
	type Component = T;
}

impl<T: Component> Colour for CieLch<T> { }

impl<T: Component> DefinedGamut for CieLch<T> { }

// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component};
use crate::hsv::Hsv;

/// An HSVA colour.
///
/// This type guarantees that its four channels -- hue, saturation, value, and alpha -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct Hsva<T>([T; 0x4]);

impl<T: Component> Hsva<T> {
	/// Constructs a new HSVA colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(hue: T, saturation: T, value: T, alpha: T) -> Self {
		let data = [hue, saturation, value, alpha];
		Self(data)
	}

	/// Discards the HSVA colour's alpha channel.
	#[inline(always)]
	#[must_use]
	pub const fn discard_alpha(self) -> (Hsv<T>, T) {
		let (hue, saturation, value, alpha) = self.get();

		let colour = Hsv::new(hue, saturation, value);
		(colour, alpha)
	}

	/// Deconstructs an HSVA colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T, T) {
		let [hue, saturation, value, alpha] = self.0;
		(hue, saturation, value, alpha)
	}
}

macro_rules! impl_conversions {
	($($tys:ty),+$(,)?) => {
		$(
			impl ::polywave::hsv::Hsva<$tys> {
				/// Converts an HWBA colour to HSVA.
				pub const fn from_hwba(colour: ::polywave::hsv::Hwba<$tys>) -> Self {
					let (colour, alpha) = colour.discard_alpha();

					let colour = Hsv::<$tys>::from_hwb(colour);

					colour.with_alpha(alpha)
				}

				/// Converts the HSVA colour to HWBA.
				pub fn to_hwba(self) -> ::polywave::hsv::Hwba<$tys> {
					let (colour, alpha) = self.discard_alpha();

					let colour = colour.to_hwb();

					colour.with_alpha(alpha)
				}
			}
		)*
	};
}

impl_conversions!(f32, f64);

#[cfg(feature = "f16")]
impl_conversions!(f16);

#[cfg(feature = "f128")]
impl_conversions!(f128);

impl<T: Component> Colour for Hsva<T> { }

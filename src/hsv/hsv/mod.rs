// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component};
use crate::hsv::{Hsva, Hwb};

/// An HSV colour.
///
/// This type guarantees that its four channels -- hue, saturation, and value -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct Hsv<T>([T; 0x3]);

impl<T: Component> Hsv<T> {
	/// Constructs a new HSV colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(hue: T, saturation: T, value: T) -> Self {
		let data = [hue, saturation, value];
		Self(data)
	}

	/// Adds an alpha channel to the HSV colour.
	#[inline(always)]
	#[must_use]
	pub const fn with_alpha(self, alpha: T) -> Hsva<T> {
		let (hue, saturation, value) = self.get();
		Hsva::new(hue, saturation, value, alpha)
	}

	/// Deconstructs an HSV colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		let [hue, saturation, value] = self.0;
		(hue, saturation, value)
	}
}

macro_rules! impl_conversions {
	($($tys:ty),+$(,)?) => {
		$(
			impl ::polywave::hsv::Hsv<$tys> {
				/// Converts an HWB colour to HSV.
				pub const fn from_hwb(colour: ::polywave::hsv::Hwb<$tys>) -> Self {
					let (hue, whiteness, blackness) = colour.get();

					let saturation = 1.0 - (whiteness / (1.0 - blackness));
					let value      = 1.0 - blackness;

					Self::new(hue, saturation, value)
				}

				/// Converts the HSV colour to HWB.
				pub fn to_hwb(self) -> ::polywave::hsv::Hwb<$tys> {
					let (hue, saturation, value) = self.get();

					let whiteness = (1.0 - saturation) * value;

					let blackness = 1.0 - value;

					Hwb::new(hue, whiteness, blackness)
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

impl<T: Component> Colour for Hsv<T> { }

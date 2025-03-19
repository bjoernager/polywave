// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::Component;
use crate::colour::{Colour, Css};

/// An RGBA colour.
///
/// This type guarantees that its four channels -- red, green, blue, and alpha -- are stored sequentially in memory (in this order).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeros, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct Rgba<T: Component>([T; 0x4]);

impl<T: Component> Rgba<T> {
	/// Constructs a new RGBA colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(red: T, green: T, blue: T, alpha: T) -> Self {
		let data = [red, green, blue, alpha];
		Self(data)
	}

	/// Deconstructs an RGBA colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T, T) {
		let [red, green, blue, alpha] = self.0;
		(red, green, blue, alpha)
	}
}

impl<T: Component> Colour for Rgba<T> {
	#[cfg(feature = "wgpu")]
	#[inline]
	fn to_wgpu_color_lossy(&self) -> wgpu_types::Color {
		// Convert perceptual RGB to linear RGB.

		let (r, g, b, a) = self.get();

		let mut r = r.to_f64_lossy();
		let mut g = g.to_f64_lossy();
		let mut b = b.to_f64_lossy();
		let     a = a.to_f64_lossy();

		for slot in [&mut r, &mut g, &mut b] {
			let mut value = *slot;

			value = if value > 0.040_450 {
				((value + 0.055) / 1.055).powf(2.4)
			} else {
				value / 12.920
			};

			*slot = value;
		}

		wgpu_types::Color { r, g, b, a }
	}
}

impl<T: Component> From<(T, T, T, T)> for Rgba<T> {
	#[inline(always)]
	fn from((red, green, blue, alpha): (T, T, T, T)) -> Self {
		Self::new(red, green, blue, alpha)
	}
}

impl From<Css> for Rgba<u8> {
	fn from(value: Css) -> Self {
		value.to_rgba()
	}
}

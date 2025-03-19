// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod test;

use crate::Component;
use crate::error::RgbaU8FromStrError;

use core::fmt::{self, Debug, Display, Formatter};
use core::ops::RangeInclusive;
use core::str::FromStr;

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
	pub const fn new(r: T, g: T, b: T, a: T) -> Self {
		let data = [r, g, b, a];
		Self(data)
	}

	/// Deconstructs an RGBA colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T, T) {
		let [r, g, b, a] = self.0;
		(r, g, b, a)
	}
}

impl Rgba<u8> {
	/// Constructs a new RGBA colour from [`u32`].
	///
	/// The `u32` value is reinterpreted as a four contiguous `u8` objects corresponding to each of the four channels.
	#[inline(always)]
	#[must_use]
	pub const fn from_u32(value: u32) -> Self {
		let data = value.to_be_bytes();
		Self(data)
	}

	/// Converts an RGBA colour to [`u32`].
	///
	/// This function is the inverse of [`from_u32`](Self::from_u32) (see there for more information).
	#[inline(always)]
	#[must_use]
	pub const fn to_u32(self) -> u32 {
		let data = self.0;
		u32::from_be_bytes(data)
	}
}

impl Display for Rgba<u8> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let value = self.to_u32();

		write!(f, "#{value:08X}")
	}
}

impl<T: Component> From<(T, T, T, T)> for Rgba<T> {
	#[inline(always)]
	fn from((r, g, b, a): (T, T, T, T)) -> Self {
		Self::new(r, g, b, a)
	}
}

impl FromStr for Rgba<u8> {
	type Err = RgbaU8FromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if !s.starts_with('#') {
			return Err(RgbaU8FromStrError::MissingHash);
		}

		let get_int_in_range = |range: RangeInclusive<usize>| -> Result<u8, Self::Err> {
			let value = s.get(range).map(|s| u8::from_str_radix(s, 0x10));

			if let Some(Ok(value)) = value {
				Ok(value)
			} else {
				Err(RgbaU8FromStrError::UnknownFormat)
			}
		};

		let colour = match s.len() {
			0x4 => {
				let r = get_int_in_range(0x1..=0x1)? * 0x11;
				let g = get_int_in_range(0x2..=0x2)? * 0x11;
				let b = get_int_in_range(0x3..=0x3)? * 0x11;

				Self::new(r, g, b, 0xFF)
			}

			0x7 => {
				let r = get_int_in_range(0x1..=0x2)?;
				let g = get_int_in_range(0x3..=0x4)?;
				let b = get_int_in_range(0x5..=0x6)?;

				Self::new(r, g, b, 0xFF)
			}

			0x9 => {
				let r = get_int_in_range(0x1..=0x2)?;
				let g = get_int_in_range(0x3..=0x4)?;
				let b = get_int_in_range(0x5..=0x6)?;
				let a = get_int_in_range(0x7..=0x8)?;

				Self::new(r, g, b, a)
			}

			len => return Err(RgbaU8FromStrError::InvalidLength(len)),
		};

		Ok(colour)
	}
}

// NOTE: We require `Into<f64>` as that also guran-
// tees the losslessness of `to_f64_lossy`.
#[cfg(feature = "wgpu")]
impl<T: Component + Into<f64>> From<Rgba<T>> for wgpu_types::Color {
	#[inline]
	fn from(value: Rgba<T>) -> Self {
		let (r, g, b, a) = value.get();

		let r = r.to_f64_lossy();
		let g = g.to_f64_lossy();
		let b = b.to_f64_lossy();
		let a = a.to_f64_lossy();

		Self { r, g, b, a }
	}
}

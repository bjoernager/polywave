// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{BalancedColour, Colour, Component, DefinedGamut};
use crate::rgb::Rgb;

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zerocopy")]
use zerocopy::{FromZeros, Immutable, IntoBytes};

/// An sRGB colour.
///
/// This type is guaranteed to always have the exact same layout as <code>[Rgb]&lt;T&gt;</code>, although with a different representation.
/// Namely, channel order is preserved in memory.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable, IntoBytes))]
pub struct SRgb<T>(Rgb<T>);

impl<T: Component> SRgb<T> {
	/// Constructs a new sRGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn new(red: T, green: T, blue: T) -> Self {
		let colour = Rgb::new(red, green, blue);
		Self(colour)
	}

	/// Reinterprets a raw RGB colour as sRGB.
	///
	/// The provided colour is *scaled* to fit the sRGB gamut.
	#[inline(always)]
	#[must_use]
	pub const fn from_rgb(colour: Rgb<T>) -> Self {
		Self(colour)
	}

	/// Maps the sRGB colour's channels.
	#[inline]
	#[must_use]
	pub fn map<U, F>(self, mut op: F) -> SRgb<U>
	where
		U: Component,
		F: FnMut(T) -> U,
	{
		let (red, green, blue) = self.get();

		let red   = op(red);
		let green = op(green);
		let blue  = op(blue);

		SRgb::new(red, green, blue)
	}

	/// Reinterprets the sRGB colour as a raw RGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn as_rgb(self) -> Rgb<T> {
		self.0
	}

	/// Deconstructs the sRGB colour.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (T, T, T) {
		self.as_rgb().get()
	}
}

macro_rules! impl_conversions {
	($($tys:ty),+$(,)?) => {
		$(
			impl ::polywave::rgb::SRgb<$tys> {
				/// Transfers a raw RGB value to perceptual RGB, scaling to the sRGB gamut.
				///
				/// The transfer is done as by the sRGB transfer function.
				#[cfg(feature = "std")]
				#[inline]
				#[must_use]
				pub fn transfer(colour: Rgb<$tys>) -> Self {
					let colour = colour.map(|mut colour| {
						let sign = colour;

						colour = colour.abs();

						colour = if colour > 0.003_130_800 {
							colour.powf(const { 1.0 / 2.4 }) * 1.055 - 0.055
						} else {
							colour * 12.920
						};

						colour = colour.copysign(sign);

						colour
					});

					Self::from_rgb(colour)
				}

				/// "Untransfers" the gamma-encoded sRGB.
				///
				/// sRGB channels are encoded using the [*transfer*](https://en.wikipedia.org/wiki/SRGB#Transfer_function_(%22gamma%22)) function (see [`transfer`](Self::transfer)).
				/// This method serves as the inverse of this function.
				///
				/// Note that the returned value is no longer sRGB as sRGB is strictly gamma-encoded.
				#[cfg(feature = "std")]
				#[inline]
				#[must_use]
				pub fn untransfer(self) -> Rgb<$tys> {
					self.as_rgb().map(|mut colour| {
						let sign = colour;

						colour = colour.abs();

						colour = if colour > 0.040_450 {
							((colour + 0.055) / 1.055).powf(2.4)
						} else {
							colour / 12.920
						};

						colour = colour.copysign(sign);

						colour
					})
				}
			}
		)*
	};
}

#[cfg(feature = "f16")]
impl_conversions!(f16);

impl_conversions!(f32, f64);

#[cfg(feature = "f128")]
impl_conversions!(f128);

unsafe impl<T: Component> BalancedColour for SRgb<T> {
	type Component = T;
}

impl<T: Component> Colour for SRgb<T> { }

impl<T: Component> DefinedGamut for SRgb<T> { }

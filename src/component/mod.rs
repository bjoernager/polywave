// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod seal {
	/// Denotes a type suitable for use as a colour component.
	pub trait Component: Copy + Sized { }
}

pub(crate) use seal::Component as SealedComponent;

/// Denotes a type suitable for use as a colour component.
///
/// Importantly, all components must be
///
/// Note that not all components are necessarily transformable; most colour spaces and formats require some form of floating- or fixed-point arithmetic for transformations to be at least be somewhat accurate.
/// Only the most basic of colours spaces can thus be translated using simple integers.
pub trait Component: SealedComponent + Copy + Sized {
	/// Lossily converts the component to [`f16`].
	///
	/// If `Self` additionally implements <code>[Into]&lt;f16&gt;</code>, then this method must also be lossless.
	#[cfg(feature = "f16")]
	#[must_use]
	fn to_f16_lossy(self) -> f16;

	/// Lossily converts the component to [`f32`].
	///
	/// If `Self` additionally implements <code>[Into]&lt;f32&gt;</code>, then this method must also be lossless.
	#[must_use]
	fn to_f32_lossy(self) -> f32;

	/// Lossily converts the component to [`f64`].
	///
	/// If `Self` additionally implements <code>[Into]&lt;f64&gt;</code>, then this method must also be lossless.
	#[must_use]
	fn to_f64_lossy(self) -> f64;

	/// Lossily converts the component to [`f128`].
	///
	/// If `Self` additionally implements <code>[Into]&lt;f128&gt;</code>, then this method must also be lossless.
	#[cfg(feature = "f128")]
	#[must_use]
	fn to_f128_lossy(self) -> f128;
}

macro_rules! impl_integer_component {
	{
		$(
			$(#[$attrs:meta])*
			$tys:ty
		),+$(,)?
	} => {
		$(
			$(#[$attrs])*
			impl ::polywave::SealedComponent for $tys { }

			$(#[$attrs])*
			impl ::polywave::Component for $tys {
				#[cfg(feature = "f16")]
				#[inline(always)]
				fn to_f16_lossy(self) -> f16 {
					self as f16 / Self::MAX as f16
				}

				#[inline(always)]
				fn to_f32_lossy(self) -> f32 {
					self as f32 / Self::MAX as f32
				}

				#[inline(always)]
				fn to_f64_lossy(self) -> f64 {
					self as f64 / Self::MAX as f64
				}

				#[cfg(feature = "f128")]
				#[inline(always)]
				fn to_f128_lossy(self) -> f128 {
					self as f128 / Self::MAX as f128
				}
			}
		)*
	};
}

macro_rules! impl_float_component {
	{
		$(
			$(#[$attrs:meta])*
			$tys:ty
		),+$(,)?
	} => {
		$(
			$(#[$attrs])*
			impl ::polywave::SealedComponent for $tys { }

			$(#[$attrs])*
			impl ::polywave::Component for $tys {
				#[cfg(feature = "f16")]
				#[inline(always)]
				fn to_f16_lossy(self) -> f16 {
					self as f16
				}

				#[inline(always)]
				fn to_f32_lossy(self) -> f32 {
					self as f32
				}

				#[inline(always)]
				fn to_f64_lossy(self) -> f64 {
					self as f64
				}

				#[cfg(feature = "f128")]
				#[inline(always)]
				fn to_f128_lossy(self) -> f128 {
					self as f128
				}
			}
		)*
	};
}

impl_integer_component! {
	u8,
	i8,
	u16,
	i16,
	u32,
	i32,
	u64,
	i64,
	u128,
	i128,
}

impl_float_component! {
	f32,
	f64,

	#[cfg(feature = "f16")]
	f16,

	#[cfg(feature = "f128")]
	f128,
}

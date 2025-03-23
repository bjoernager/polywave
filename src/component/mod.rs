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
/// Note that not all components are necessarily transformable; most colour spaces and formats require some form of floating- or fixed-point arithmetic for transformations to be at least be somewhat accurate.
/// Only the most basic of colours spaces can thus be translated using simple integers.
pub trait Component: SealedComponent + Copy + Sized { }

macro_rules! impl_component {
	{
		$(
			$(#[$attrs:meta])*
			$tys:ty
		),*$(,)?
	} => {
		$(
			$(#[$attrs])*
			impl ::polywave::SealedComponent for $tys { }

			$(#[$attrs])*
			impl ::polywave::Component for $tys { }
		)*
	};
}

impl_component! {
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

	f32,
	f64,

	#[cfg(feature = "f16")]
	f16,

	#[cfg(feature = "f128")]
	f128,
}

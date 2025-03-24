// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

/// Denotes a type suitable for use as a colour component.
///
/// Note that not all components types are necessarily transformable; most colour spaces and formats require some form of floating- or fixed-point arithmetic for transformations to be at least be somewhat accurate.
/// Only the most basic of colours spaces can thus be translated using simple integers.
///
/// Also note that some colour types may be preprogrammed for specific components and are thus not generic or only generic for a specific subset of components types.
/// Usually, this trait is only useful for types that implement [`BalancedColour`](crate::BalancedColour).
///
/// # Safety
///
/// Implementors of this type may not contain any padding.
///
/// Additionally, the component must also have a bit state for denoting `0` (zero), and this state must only contain null bits.
pub unsafe trait Component: Copy + Sized { }

macro_rules! impl_component {
	{
		$(
			$(#[$attrs:meta])*
			$tys:ty
		),*$(,)?
	} => {
		$(
			$(#[$attrs])*
			unsafe impl ::polywave::Component for $tys { }
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

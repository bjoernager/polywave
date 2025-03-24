// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::mem::{offset_of, ManuallyDrop};

use crate::{BalancedColour, Colour, DefinedGamut};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zerocopy")]
use zerocopy::{FromZeros, Immutable, IntoBytes};

/// Denotes a colour with an attached alpha channel.
///
/// Most (if not all) colour spaces define the same semantics for the alpha channel, partly due to the very intent of an alpha channel being basically universal.
/// For this reason, the alpha channel contained in this type will always be stored **last** in memory, and it will also always be linearly encoded.
///
/// If another, specific representation is desired, then a custom, complete type should be defined instead.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde",    derive(Deserialize, Serialize))]
#[cfg_attr(feature = "zerocopy", derive(FromZeros, Immutable))]
pub struct Alpha<T: BalancedColour> {
	colour: T,
	alpha:  T::Component,
}

impl<T: BalancedColour> Alpha<T> {
	/// Attaches an alpha channel to a colour.
	#[inline(always)]
	#[must_use]
	pub const fn attach(colour: T, alpha: T::Component) -> Self {
		Self { colour, alpha }
	}

	/// Detaches the colour from its alpha channel.
	#[inline(always)]
	#[must_use]
	pub const fn detach(self) -> (T, T::Component) {
		let alpha = self.alpha;

		let this = ManuallyDrop::new(self);

		// SAFETY: `ManuallyDrop<T>` is transparent to `T`.
		let ptr = &raw const this as *const Self;

		let colour = {
			let off = offset_of!(Self, colour);
			let ptr = unsafe { ptr.byte_add(off) } as *const T;

			unsafe { ptr.read() }
		};

		(colour, alpha)
	}
}

impl<T: BalancedColour> Colour for Alpha<T> { }

unsafe impl<T: BalancedColour> BalancedColour for Alpha<T> {
	type Component = T::Component;
}

impl<T: BalancedColour + DefinedGamut> DefinedGamut for Alpha<T> { }

// SAFETY: `BalancedColour` guarantees that there
// will be no extra padding between `colour` and
// `alpha`, and the `IntoBytes` bounds likewise
// guarantees the same inside `colour` and `alpha`.
#[cfg(feature = "zerocopy")]
unsafe impl<T> IntoBytes for Alpha<T>
where
	T:            BalancedColour + IntoBytes,
	T::Component: IntoBytes,
{
	fn only_derive_is_allowed_to_implement_this_trait() { }
}

// SAFETY: `BalancedColour` guarantees that there
// will be no extra padding between `colour` and
// `alpha`, and the `Pod` bounds likewise guaran-
// tees the same inside `colour` and `alpha`.
#[cfg(feature = "bytemuck")]
unsafe impl<T> Pod for Alpha<T>
where
	T:            BalancedColour + Pod,
	T::Component: Pod,
{ }

#[cfg(feature = "bytemuck")]
unsafe impl<T> Zeroable for Alpha<T>
where
	T:            BalancedColour + Zeroable,
	T::Component: Zeroable,
{ }

// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Colour, Component};

/// Denotes a "balanced" colour.
///
/// Types that implement this type state that their representation is build up of equal segments of some component type (see [`Component`](Self::Component))
///
/// This trait is mainly useful when the underlying layout of [`Alpha`](crate::Alpha) needs to be guaranteed.
///
/// # Safety
///
/// Implementing this type allows users to always assume that:
///
/// * `<Self as BalancedColour>::Component` is the actual type used to represent the underlying channel(s),
/// * `<Self as BalancedColour>::Component` itself contains no padding (i.e. it is also "balanced")<sup>&dagger;</sup>, and
/// * `Self` additionally has the a layout equivalent to that of `[Self::Component; N]` for some constant `N` that is the channel count.
///
/// If any of these requirements are *not* upheld, then behaviour is undefined.
///
/// <sub>&dagger; Note that the [`Component`] trait already requires this.</sub>
pub unsafe trait BalancedColour: Colour {
	/// The underlying type used by `Self` to represent colour channels.
	type Component: Component;
}

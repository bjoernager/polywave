// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

/// Denotes a colour.
pub trait Colour {
	/// Lossily converts the colour to [`wgpu::Color`](wgpu_types::Color).
	#[cfg(feature = "wgpu")]
	#[must_use]
	fn to_wgpu_color_lossy(&self) -> wgpu_types::Color;
}

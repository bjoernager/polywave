// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::fmt::{self, Debug, Display, Formatter};

type Buffer = [u8; 0x4];

#[repr(align(0x4), C)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern, bytemuck::NoUninit))]
#[cfg_attr(feature = "serde",    derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::FromBytes, zerocopy::Immutable, zerocopy::IntoBytes))]
pub struct Srgba8(Buffer);

impl Srgba8 {
	#[inline(always)]
	#[must_use]
	pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		let buf = [r, g, b, a];
		Self(buf)
	}

	#[inline(always)]
	#[must_use]
	pub const fn from_u32(value: u32) -> Self {
		let buf = value.to_be_bytes();
		Self(buf)
	}

	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (u8, u8, u8, u8) {
		let [r, g, b, a] = self.0;
		(r, g, b, a)
	}

	#[inline(always)]
	#[must_use]
	pub const fn into_u32(self) -> u32 {
		let data = self.0;
		u32::from_be_bytes(data)
	}
}

impl Debug for Srgba8 {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.get(), f)
	}
}

impl Display for Srgba8 {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "#{:08X}", self.into_u32())
	}
}

impl From<(u8, u8, u8, u8)> for Srgba8 {
	#[inline(always)]
	fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
		Self::new(r, g, b, a)
	}
}

impl From<u32> for Srgba8 {
	#[inline(always)]
	fn from(value: u32) -> Self {
		Self::from_u32(value)
	}
}

#[cfg(feature = "wgpu")]
impl From<Srgba8> for wgpu_types::Color {
	#[inline]
	fn from(value: Srgba8) -> Self {
		let (r, g, b, a) = value.get();

		Self {
			r: f64::from(r) / f64::from(u8::MAX),
			g: f64::from(g) / f64::from(u8::MAX),
			b: f64::from(b) / f64::from(u8::MAX),
			a: f64::from(a) / f64::from(u8::MAX),
		}
	}
}

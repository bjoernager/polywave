// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![cfg(test)]

use crate::Rgba;

use alloc::format;
use core::str::FromStr;

#[test]
fn test_rgba_u8_display() {
	assert_eq!(format!("{}", Rgba::<u8>::from_u32(0xF0F8FFFF)), "#F0F8FFFF");
}

#[test]
fn test_rgba_u8_from_str() {
	assert_eq!(Rgba::<u8>::from_str("#639"), Ok(Rgba::new(0x66, 0x33, 0x99, 0xFF)));

	assert_eq!(Rgba::<u8>::from_str("#00FF7F"), Ok(Rgba::new(0x00, 0xFF, 0x7F, 0xFF)));

	assert_eq!(Rgba::<u8>::from_str("#FfD7007f"), Ok(Rgba::new(0xFF, 0xD7, 0x00, 0x7F)));
}

#[test]
fn test_rgba_u8_from_u32() {
	assert_eq!(Rgba::<u8>::from_u32(0x80808080), Rgba::<u8>::new(0x80, 0x80, 0x80, 0x80));
}

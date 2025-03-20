// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![cfg(test)]

use crate::www::Html;

use alloc::format;
use core::str::FromStr;

#[test]
fn test_html_display() {
	assert_eq!(format!("{}", Html::from_u32(0xF0F8FFFF)), "#F0F8FFFF");
}

#[test]
fn test_html_from_str() {
	assert_eq!(Html::from_str("#639"), Ok(Html::new(0x66, 0x33, 0x99, 0xFF)));

	assert_eq!(Html::from_str("#00FF7F"), Ok(Html::new(0x00, 0xFF, 0x7F, 0xFF)));

	assert_eq!(Html::from_str("#FfD7007f"), Ok(Html::new(0xFF, 0xD7, 0x00, 0x7F)));

	assert_eq!(Html::from_str("transparent"), Ok(Html::new(0x00, 0x00, 0x00, 0x00)));
}

#[test]
fn test_html_from_u32() {
	assert_eq!(Html::from_u32(0x80808080), Html::new(0x80, 0x80, 0x80, 0x80));
}

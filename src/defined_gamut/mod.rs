// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::Colour;

/// Denotes a colour with a well-defined gamut.
///
/// Types such as [`Rgb`](crate::rgb::Rgb) do not define this trait as they their gamuts are relative to the final display.
///
/// Note that colours that define this trait may still be further transformed before being displayed.
pub trait DefinedGamut: Colour { }

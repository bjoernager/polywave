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
/// More specifically, this trait denotes colours that have an "ideal" representation, i.e. each well-defined value can, in theory, only be displayed in one way.
/// Examples include [`CieXyz`](crate::xyz::CieXyz) and [`OpRgb`](crate::rgb::OpRgb).
///
/// Types such as [`Rgb`](crate::rgb::Rgb) do not define this trait as their gamuts are unspecified.
/// Note that colours that do define this trait may still be further transformed before being displayed.
pub trait DefinedGamut: Colour { }

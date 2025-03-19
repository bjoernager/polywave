// Copyright 2025 Gabriel Bjørnager Jensen.

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A RGBA colour code could not be parsed.
#[derive(Debug, Eq, PartialEq)]
pub enum RgbaU8FromStrError {
	/// The RGBA colour code had an invalid length.
	InvalidLength(usize),

	/// The RGBA colour code had no prefixed hash `#`.
	MissingHash,

	/// The RGBA colour code had an otherwise unknown format.
	///
	/// This error is only emitted if neither of the two others is appropriate.
	UnknownFormat,
}

impl Error for RgbaU8FromStrError { }

impl Display for RgbaU8FromStrError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::InvalidLength(len)
			=> write!(f, "rgba code has length `{len}` but should have been neither `4` or `7` or `9` octets long"),

			Self::MissingHash
			=> write!(f, "rgba code is missing prefixed hash `#`"),

			Self::UnknownFormat
			=> write!(f, "rgba code is of an otherwise unknown format"),
		}
	}
}

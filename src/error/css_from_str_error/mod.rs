// Copyright 2025 Gabriel Bjørnager Jensen.

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A [CSS colour](crate::colour::Css) could not be parsed.
#[derive(Debug, Eq, PartialEq)]
pub enum CssFromStrError {
	/// The CSS colour was missing a hash `#`.
	MissingHash,

	/// The CSS colour had an otherwise unknown format.
	UnknownFormat,
}

impl Error for CssFromStrError { }

impl Display for CssFromStrError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::MissingHash
			=> write!(f, "css colour is missing prefixed hash `#`"),

			Self::UnknownFormat
			=> write!(f, "css colour is of an otherwise unknown format"),
		}
	}
}

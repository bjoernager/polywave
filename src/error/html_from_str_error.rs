// Copyright 2025 Gabriel Bjørnager Jensen.

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// An [HTML colour](crate::www::Html) could not be parsed.
#[derive(Debug, Eq, PartialEq)]
pub enum HtmlFromStrError {
	/// An HTML hexadecimal colour was missing a hash `#`.
	MissingHash,

	/// An HTML colour had an otherwise unknown format.
	UnknownFormat,

	/// An HTML named colour was unknown.
	UnknownName,
}

impl Error for HtmlFromStrError { }

impl Display for HtmlFromStrError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::MissingHash
			=> write!(f, "html hexadecimal colour is missing prefixed hash `#`"),

			Self::UnknownFormat
			=> write!(f, "html colour is of an otherwise unknown format"),

			Self::UnknownName
			=> write!(f, "html named colour is unknown"),
		}
	}
}

// Copyright 2025 Gabriel Bjørnager Jensen.

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A [HTML colour](crate::www::Html) could not be parsed.
#[derive(Debug, Eq, PartialEq)]
pub enum HtmlFromStrError {
	/// The HTML colour was missing a hash `#`.
	MissingHash,

	/// The HTML colour had an otherwise unknown format.
	UnknownFormat,
}

impl Error for HtmlFromStrError { }

impl Display for HtmlFromStrError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::MissingHash
			=> write!(f, "html colour is missing prefixed hash `#`"),

			Self::UnknownFormat
			=> write!(f, "html colour is of an otherwise unknown format"),
		}
	}
}

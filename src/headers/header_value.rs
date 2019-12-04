use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use crate::Mime;

/// A header value.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct HeaderValue {
    inner: String,
}

impl HeaderValue {
    /// Create a new `HeaderValue` from ASCII bytes.
    ///
    /// # Error
    ///
    /// This function will error if the string
    pub fn from_ascii(bytes: &[u8]) -> Result<Self, ParseError> {
        if !bytes.is_ascii() {
            return Err(ParseError { _private: () });
        }

        // This is permitted because ASCII is valid UTF-8, and we just checked that.
        let string = unsafe { String::from_utf8_unchecked(bytes.to_ascii_lowercase()) };
        Ok(Self { inner: string })
    }

    /// Converts a vector of bytes to a `HeaderValue` without checking that the string contains
    /// valid ASCII.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed to it are valid
    /// ASCII. If this constraint is violated, it may cause memory
    /// unsafety issues with future users of the HeaderValue, as the rest of the library assumes
    /// that Strings are valid ASCII.
    pub unsafe fn from_ascii_unchecked(bytes: Vec<u8>) -> Self {
        let string = String::from_utf8_unchecked(bytes);
        Self { inner: string }
    }
}

impl From<Mime> for HeaderValue {
    fn from(mime: Mime) -> Self {
        if let Some(string) = mime.static_str {
            HeaderValue {
                inner: string.to_string(),
            }
        } else {
            HeaderValue { inner: mime.string }
        }
    }
}

/// An error returned when failing to convert into a header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseError {
    _private: (),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Error parsing a string into a status code".fmt(f)
    }
}

impl FromStr for HeaderValue {
    type Err = ParseError;

    /// Create a new `HeaderValue`.
    ///
    /// This checks it's valid ASCII, and lowercases it.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ParseError { _private: () });
        }
        Ok(Self {
            inner: s.to_ascii_lowercase(),
        })
    }
}
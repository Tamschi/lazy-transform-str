//! Lazy-copying lazy-allocated scanning [`str`] transformations.  
//! This is good e.g. for (un)escaping text, especially if individual strings are short.
//!
//! Note that this library uses [smartstring] (and as such returns [`Woc`]s instead of [`Cow`]s).  
//! The output is still [`Deref<Target = str>`] regardless, so there should be no issue with ease of use.
//!
//! # Example
//!
//! ```rust
//! use {
//!     gnaw::Unshift as _,
//!     lazy_transform_str::{Transform as _, TransformedPart},
//!     smartstring::alias::String,
//!     woc::Woc,
//! };
//!
//! fn double_a(str: &str) -> Woc<String, str> {
//!     str.transform(|rest /*: &mut &str */| {
//!         // Consume some of the input. `rest` is never empty here.
//!         match rest.unshift().unwrap() {
//!             'a' => TransformedPart::Changed(String::from("aa")),
//!             _ => TransformedPart::Unchanged,
//!         }
//!     } /*: impl FnMut(…) -> … */ )
//! }
//!
//! assert_eq!(double_a("abc"), Woc::Owned(String::from("aabc")));
//! assert_eq!(double_a("bcd"), Woc::Borrowed("bcd"));
//! ```
//!
//! See [`escape_double_quotes`] and [`unescape_backlashed_verbatim`]'s sources for more real-world examples.

#![warn(clippy::pedantic)]

use gnaw::Unshift as _;
use smartstring::alias::String;
use woc::Woc;

pub enum TransformedPart {
	Unchanged,
	Changed(String),
}

pub fn transform(
	str: &str,
	transform_next: impl FnMut(/* rest: */ &mut &str) -> TransformedPart,
) -> Woc<String, str> {
	str.transform(transform_next)
}

pub trait Transform {
	fn transform(
		&self,
		transform_next: impl FnMut(&mut &str) -> TransformedPart,
	) -> Woc<String, str>;
}

impl Transform for str {
	fn transform(
		&self,
		mut transform_next: impl FnMut(&mut &str) -> TransformedPart,
	) -> Woc<String, str> {
		let mut rest = self;
		let mut copied = loop {
			if rest.is_empty() {
				return Woc::Borrowed(self);
			}
			let unchanged_rest = rest;
			if let TransformedPart::Changed(transformed) = transform_next(&mut rest) {
				let mut copied = String::from(&self[..self.len() - unchanged_rest.len()]);
				copied.push_str(&transformed);
				break copied;
			}
		};

		while !rest.is_empty() {
			let unchanged_rest = rest;
			match transform_next(&mut rest) {
				TransformedPart::Unchanged => {
					copied.push_str(&unchanged_rest[..unchanged_rest.len() - rest.len()]);
				}
				TransformedPart::Changed(changed) => copied.push_str(&changed),
			}
		}

		Woc::Owned(copied)
	}
}

#[must_use = "pure function"]
pub fn escape_double_quotes(string: &str) -> Woc<String, str> {
	string.transform(|rest| match rest.unshift().unwrap() {
		c @ '\\' | c @ '"' => {
			let mut changed = String::from(r"\");
			changed.push(c);
			TransformedPart::Changed(changed)
		}
		_ => TransformedPart::Unchanged,
	})
}

#[must_use = "pure function"]
pub fn unescape_backslashed_verbatim(string: &str) -> Woc<String, str> {
	let mut escaped = false;
	string.transform(|rest| match rest.unshift().unwrap() {
		'\\' if !escaped => {
			escaped = true;
			TransformedPart::Changed(String::new())
		}
		_ => {
			escaped = false;
			TransformedPart::Unchanged
		}
	})
}

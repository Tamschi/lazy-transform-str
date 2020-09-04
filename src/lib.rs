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
//!     cervine::Cow,
//!     gnaw::Unshift as _,
//!     lazy_transform_str::{Transform as _, TransformedPart},
//!     smartstring::alias::String,
//! };
//!
//! fn double_a(str: &str) -> Cow<String, str> {
//!     str.transform(|rest /*: &mut &str */| {
//!         // Consume some of the input. `rest` is never empty here.
//!         match rest.unshift().unwrap() {
//!             'a' => TransformedPart::Changed(String::from("aa")),
//!             _ => TransformedPart::Unchanged,
//!         }
//!     } /*: impl FnMut(…) -> … */ )
//! }
//!
//! assert_eq!(double_a("abc"), Cow::Owned(String::from("aabc")));
//! assert_eq!(double_a("bcd"), Cow::Borrowed("bcd"));
//! ```
//!
//! See [`escape_double_quotes`] and [`unescape_backlashed_verbatim`]'s sources for more real-world examples.

#![warn(clippy::pedantic)]
#![doc(html_root_url = "https://docs.rs/lazy-transform-str/0.0.2")]

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

use cervine::Cow;
use gnaw::Unshift as _;
use smartstring::alias::String;

/// Inidicates whether the consumed part of the input remains unchanged or is to be replaced.
pub enum TransformedPart {
	Unchanged,
	Changed(String),
}

/// Transforms the given `str` according to `transform_next` as lazily as possible.
///
/// With each invocation, `transform_next` should consume part of the input (by slicing its parameter in place) and return a replacement [`String`] if necessary.
/// `transform` returns once the input is an empty [`str`].
///
/// [`String`]: https://doc.rust-lang.org/stable/std/string/struct.String.html
/// [`str`]: https://doc.rust-lang.org/stable/std/primitive.str.html
///
/// # Example
///
/// ```rust
/// use cervine::Cow;
/// use gnaw::Unshift as _;
/// use lazy_transform_str::{transform, TransformedPart};
/// use smartstring::alias::String;
///
/// let input = r#"a "quoted" word"#;
///
/// // Escape double quotes
/// let output = transform(input, |rest| match rest.unshift().unwrap() {
///     c @ '\\' | c @ '"' => {
///         let mut changed = String::from(r"\");
///         changed.push(c);
///         TransformedPart::Changed(changed)
///     }
///     _ => TransformedPart::Unchanged,
/// });
///
/// assert_eq!(output, Cow::Owned(r#"a \"quoted\" word"#.into()));
/// ```
pub fn transform(
	str: &str,
	transform_next: impl FnMut(/* rest: */ &mut &str) -> TransformedPart,
) -> Cow<String, str> {
	str.transform(transform_next)
}

/// Helper trait to call [`transform`] as method on [`&str`].
///
/// [`transform`]: ./fn.transform.html
/// [`&str`]: https://doc.rust-lang.org/stable/std/primitive.str.html
///
/// # Example
///
/// ```rust
/// use cervine::Cow;
/// use gnaw::Unshift as _;
/// use lazy_transform_str::{Transform as _, TransformedPart};
/// use smartstring::alias::String;
///
/// let input = r#"a "quoted" word"#;
///
/// // Escape double quotes
/// let output = input.transform(|rest| match rest.unshift().unwrap() {
///     c @ '\\' | c @ '"' => {
///         let mut changed = String::from(r"\");
///         changed.push(c);
///         TransformedPart::Changed(changed)
///     }
///     _ => TransformedPart::Unchanged,
/// });
///
/// assert_eq!(output, Cow::Owned(r#"a \"quoted\" word"#.into()));
/// ```
pub trait Transform {
	fn transform(
		&self,
		transform_next: impl FnMut(&mut &str) -> TransformedPart,
	) -> Cow<String, str>;
}

impl Transform for str {
	fn transform(
		&self,
		mut transform_next: impl FnMut(&mut &str) -> TransformedPart,
	) -> Cow<String, str> {
		let mut rest = self;
		let mut copied = loop {
			if rest.is_empty() {
				return Cow::Borrowed(self);
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

		Cow::Owned(copied)
	}
}

/// Replaces `\` and `"` in `string` with (repectively) `\\` and `\"`, as lazily as possible.
///
/// # Example
///
/// ```rust
/// use cervine::Cow;
/// use lazy_transform_str::escape_double_quotes;
///
/// let input = r#"a "quoted" word"#;
///
/// let output = escape_double_quotes(input);
///
/// assert_eq!(output, Cow::Owned(r#"a \"quoted\" word"#.into()));
/// ```
#[must_use = "pure function"]
pub fn escape_double_quotes(string: &str) -> Cow<String, str> {
	string.transform(|rest| match rest.unshift().unwrap() {
		c @ '\\' | c @ '"' => {
			let mut changed = String::from(r"\");
			changed.push(c);
			TransformedPart::Changed(changed)
		}
		_ => TransformedPart::Unchanged,
	})
}

/// Replaces `\` followed by any Unicode [`char`] in `string` with that [`char`], as lazily as possible.  
/// If `\\` is found, this sequence is consumed at once and a single `\` remains in the output.
///
/// [`char`]: https://doc.rust-lang.org/stable/std/primitive.char.html
///
/// # Example
///
/// ```rust
/// use cervine::Cow;
/// use lazy_transform_str::unescape_backslashed_verbatim;
///
/// let input = r#"A \"quoted\" word\\!"#;
///
/// let output = unescape_backslashed_verbatim(input);
///
/// assert_eq!(output, Cow::Owned(r#"A "quoted" word\!"#.into()));
///
/// let output = unescape_backslashed_verbatim(&output);
///
/// assert_eq!(output, Cow::Owned(r#"A "quoted" word!"#.into()));
/// ```
#[must_use = "pure function"]
pub fn unescape_backslashed_verbatim(string: &str) -> Cow<String, str> {
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

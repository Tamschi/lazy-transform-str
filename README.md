# lazy-transform-str

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/lazy-transform-str)
[![Crates.io](https://img.shields.io/crates/v/lazy-transform-str)](https://crates.io/crates/lazy-transform-str)
[![Docs.rs](https://docs.rs/lazy-transform-str/badge.svg)](https://docs.rs/crates/lazy-transform-str)

![Rust 1.42.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.42.0&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/lazy-transform-str.svg?branch=develop)](https://travis-ci.com/Tamschi/lazy-transform-str/branches)
![Crates.io - License](https://img.shields.io/crates/l/lazy-transform-str/0.0.4)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/lazy-transform-str)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/lazy-transform-str)](https://github.com/Tamschi/lazy-transform-str/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/lazy-transform-str)](https://github.com/Tamschi/lazy-transform-str/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/lazy-transform-str.svg)](https://web.crev.dev/rust-reviews/crate/lazy-transform-str/)

Lazy-copying lazy-allocated scanning `str` transformations.

This is good e.g. for (un)escaping text, especially if individual strings are short.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add lazy-transform-str
```

## Example

```rust
use {
    cervine::Cow,
    gnaw::Unshift as _,
    lazy_transform_str::{Transform as _, TransformedPart},
    smartstring::alias::String,
};

fn double_a(str: &str) -> Cow<String, str> {
    str.transform(|rest /*: &mut &str */| {
        // Consume some of the input. `rest` is never empty here.
        match rest.unshift().unwrap() {
            'a' => TransformedPart::Changed(String::from("aa")),
            _ => TransformedPart::Unchanged,
        }
    } /*: impl FnMut(…) -> … */ )
}

assert_eq!(double_a("abc"), Cow::Owned(String::from("aabc")));
assert_eq!(double_a("bcd"), Cow::Borrowed("bcd"));
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`lazy-transform-str` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).

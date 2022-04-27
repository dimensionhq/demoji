//! A lightweight and snappy crate to remove emojis from a string.
//!
//! ## Overview
//!
//! This crate allows for removal of emojis given a `String`, and that's it!
//!
//! This crate aims to be:
//!
//! - Fast
//! - Lightweight
//! - Easy To Use
//!
//! ## Examples
//!
//! ```rust
//! # use demoji::demoji;
//! // Remove all emojis from this string
//! let demojified_string = demoji("⚡hel✅🙂lo🙂");
//! assert_eq!(demojified_string, "hello");
//! ```
use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;

/// Removes all emojis from a string **(retains chinese characters)**
///
/// # Arguments
///
/// * `string` - String with emojis
///
/// # Returns
///
/// * `Cow<str>` - De-emojified string
///
/// If no emojis are found in the given string, then the original string is returned, avoiding the allocation of a new string.
///
/// # Examples
///
/// ```
/// # use demoji::demoji;
/// // Remove all emojis from this string
/// let demojified_string = demoji("⚡hel✅🙂lo🙂");
/// assert_eq!(demojified_string, String::from("hello"));
/// // Output: `hello`
/// ```
pub fn demoji(string: &str) -> Cow<'_, str> {
    // cache regex to avoid re-compiling it every call
    static REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(concat!(
            "[",
            "\u{01F600}-\u{01F64F}", // emoticons
            "\u{01F300}-\u{01F5FF}", // symbols & pictographs
            "\u{01F680}-\u{01F6FF}", // transport & map symbols
            "\u{01F1E0}-\u{01F1FF}", // flags (iOS)
            "\u{002702}-\u{0027B0}",
            "\u{0024C2}-\u{01F251}",
            "]+",
        ))
        .unwrap()
    });

    REGEX.replace_all(&string, "")
}

/// Demojify `String` and `&str`
pub trait Demoji {
    fn demojify(&self) -> Cow<'_, str>;
}

impl Demoji for &str {
    fn demojify(&self) -> Cow<'_, str> {
        demoji(self)
    }
}

impl Demoji for String {
    fn demojify(&self) -> Cow<'_, str> {
        demoji(&self)
    }
}

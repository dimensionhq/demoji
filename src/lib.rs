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
//! // Remove all emojis from this string
//! let demojified_string = demoji("⚡hel✅🙂lo🙂");
//! assert_eq!(demojified_string, "hello");
//! ```
use regex::Regex;

/// Removes all emojis from a string **(retains chinese characters)**
///
/// # Arguments
///
/// * `string` - String with emojis
///
/// # Returns
///
/// * `String` - De-emojified string
///
/// # Examples
///
/// ```
///
/// // Remove all emojis from this string
/// let demojified_string = demoji("⚡hel✅🙂lo🙂")
/// assert_eq!(demojified_string, "hello");
/// // Output: `hello`
/// ```
pub fn demoji(string: &str) -> String {
    let regex = Regex::new(concat!(
        "[",
        "\u{01F600}-\u{01F64F}", // emoticons
        "\u{01F300}-\u{01F5FF}", // symbols & pictographs
        "\u{01F680}-\u{01F6FF}", // transport & map symbols
        "\u{01F1E0}-\u{01F1FF}", // flags (iOS)
        "\u{002702}-\u{0027B0}",
        "\u{0024C2}-\u{01F251}",
        "]+",
    ))
    .unwrap();

    regex.replace_all(&string, "").to_string()
}

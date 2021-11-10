# Demoji

A lightweight and snappy crate to remove emojis from a string.

## Overview

This crate allows for removal of emojis given a string, and that's it!

This crate aims to be:

- Fast
- Lightweight
- Easy To Use

## Examples

```rust
// Remove all emojis from this string
let demojified_string = demoji("⚡hel✅🙂lo🙂")
assert_eq!(demojified_string, "hello");
```

### What does the name mean?

Well, you could interpret it as `demolish-emoji` or `de-emoji` 😁.

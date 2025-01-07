# Homebrew Rust Package

`[Homebrew](https://brew.sh/)` 在 `Rust` 中的封装

## 使用

通过 [examples](https://github.com/wxnacy/homebrew-rs/tree/master/examples) 来了解 homebrew-rs 的使用

```rust
extern crate homebrew as brew;

/// cargo run --example info
///
/// {
///   "casks": [],
///   "formulae": [
///     {
///       "name": "wget",
///       "full_name": "wget",
///       "tap": "homebrew/core",
///       "oldnames": [],
///       "aliases": [],
///       "versioned_formulae": [],
///       "desc": "Internet file retriever",
///       "license": "GPL-3.0-or-later",
///       "homepage": "https://www.gnu.org/software/wget/",
///     ...
///     }
///   ]
/// }
fn main() {
    let pkg = brew::info("wget").expect("Failed run brew info");
    let json = serde_json::to_string_pretty(&pkg).expect("Failed to string pretty");
    println!("{json}")
}

```

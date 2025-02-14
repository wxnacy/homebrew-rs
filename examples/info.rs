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
///
///       ...
///     }
///   ]
/// }
/// package name: wget
/// package is_installed: true
fn main() {
    let pkg = brew::info("wget").unwrap();
    let json = serde_json::to_string_pretty(&pkg).unwrap();
    println!("{json}");
    println!("package name: {}", pkg.name);
    println!("package is_installed: {}", pkg.is_installed());
}

use anyhow::Result;

use crate::{brew, Package};

/// 执行 `brew info {name} --json=v2` 命令
/// 返回格式化包对象
///
/// Examples
///
/// ```
/// extern crate homebrew as brew;
///
/// fn main() {
///     let pkg = brew::info("rust").unwrap();
///
///     assert_eq!(pkg.name, "rust");
///     assert_eq!(pkg.full_name, "rust");
///     assert_eq!(pkg.tap, "homebrew/core");
///     assert_eq!(pkg.desc, "Safe, concurrent, practical language");
///     assert_eq!(pkg.homepage, "https://www.rust-lang.org/");
///     assert!(!pkg.is_cask());
///
///     let pkg = brew::info("kitty").unwrap();
///
///     assert_eq!(pkg.name, "kitty");
///     assert_eq!(pkg.full_name, "kitty");
///     assert_eq!(pkg.tap, "homebrew/cask");
///     assert_eq!(pkg.desc, "GPU-based terminal emulator");
///     assert_eq!(pkg.homepage, "https://github.com/kovidgoyal/kitty");
///     assert!(pkg.is_cask());
/// }
/// ```
pub fn info(name: &str) -> Result<Package> {
    let out = brew(format!("info {name} --json=v2").as_str())?;
    let pkg = Package::from(&out)?;
    Ok(pkg)
}

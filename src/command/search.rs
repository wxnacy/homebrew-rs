use anyhow::Result;

use crate::Brew;


/// 运行 `brew search [name]` 命令
///
/// Examples
///
/// ```
/// extern crate homebrew;
///
/// let out = homebrew::search("wget")
///     .unwrap();
///
/// assert_eq!(out, ["wget", "wget2", "wgetpaste"]);
/// ```
pub fn search(name: &str) -> Result<Vec<String>> {
    Brew::default()
        .set_cmd(format!("search {name}"))
        .output_vec()
}

use crate::{brew, brew_spawn};
use anyhow::Result;



/// 执行更新 `brew update` 命令
///
/// 在执行结束后返回结果，如果想实时输出请用 [`update_spawn`]
///
/// Examples
///
/// ```ignore
/// use homebrew;
///
/// let out = homebrew::update().unwrap();
/// println!("{out}");
/// ```
pub fn update() -> Result<String> {
    brew("update")
}

/// 执行更新 `brew update` 命令并实时输出
///
/// 实时输出信息，如果想等执行完再拿结果请用 [`update`]
///
/// Examples
///
/// ```ignore
/// use homebrew;
///
/// homebrew::update_spawn().unwrap();
/// ```
pub fn update_spawn() -> Result<()> {
    brew_spawn("update")
}

/// 安装软件 `brew install [name]` 命令
///
/// 在执行结束后返回结果，如果想实时输出请用 [`install_spawn`]
///
/// Examples
///
/// ```ignore
/// use homebrew;
///
/// homebrew::install("btop").unwrap();
/// ```
pub fn install(name: &str) -> Result<String> {
    brew(format!("install {name}").as_str())
}

/// 安装软件 `brew install [name]` 命令，并实时输出
///
/// 实时输出信息，如果想等执行完再拿结果请用 [`install`]
///
/// Examples
///
/// ```ignore
/// use homebrew;
///
/// homebrew::install_spawn("btop").unwrap();
/// ```
pub fn install_spawn(name: &str) -> Result<()> {
    brew_spawn(format!("install {name}").as_str())
}

/// 重新安装软件 `brew reinstall [name]` 命令
///
/// 在执行结束后返回结果，如果想实时输出请用 [`reinstall_spawn`]
///
/// Examples
///
/// ```ignore
/// use homebrew;
///
/// homebrew::reinstall("btop").unwrap();
/// ```
pub fn reinstall(name: &str) -> Result<String> {
    brew(format!("reinstall {name}").as_str())
}

/// 重新安装软件 `brew reinstall [name]` 命令，并实时输出
///
/// 实时输出信息，如果想等执行完再拿结果请用 [`reinstall`]
///
/// Examples
///
/// ```ignore
/// use homebrew;
///
/// homebrew::reinstall_spawn("btop").unwrap();
/// ```
pub fn reinstall_spawn(name: &str) -> Result<()> {
    brew_spawn(format!("reinstall {name}").as_str())
}

/// 卸载软件 `brew uninstall [name]` 命令
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let out = homebrew::uninstall("gotop").unwrap_err();
///
/// assert_eq!(out.to_string(), "Error: No such keg: /opt/homebrew/Cellar/gotop");
/// ```
pub fn uninstall(name: &str) -> Result<String> {
    brew(format!("uninstall {name}").as_str())
}

/// 升级软件 `brew upgrade [name]` 命令
///
/// 在执行结束后返回结果，如果想实时输出请用 [`upgrade_spawn`]
///
/// Examples
///
/// ```ignore
/// use homebrew;
///
/// homebrew::upgrade("btop").unwrap();
/// ```
pub fn upgrade(name: &str) -> Result<String> {
    brew(format!("upgrade {name}").as_str())
}

/// 升级软件 `brew upgrade [name]` 命令，并实时输出
///
/// 实时输出信息，如果想等执行完再拿结果请用 [`upgrade`]
///
/// Examples
///
/// ```ignore
/// use homebrew;
///
/// homebrew::upgrade_spawn("btop").unwrap();
/// ```
pub fn upgrade_spawn(name: &str) -> Result<()> {
    brew_spawn(format!("upgrade {name}").as_str())
}

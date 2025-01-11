use crate::{brew, Config};

/// 执行 `brew config` 命令并得到结构体
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let config = homebrew::config().unwrap();
/// assert_eq!(config.origin, "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git");
/// assert_eq!(config.prefix, "/opt/homebrew");
/// ```
pub fn config() -> anyhow::Result<Config> {
    let output = brew("config")?;
    Config::from(&output)
}

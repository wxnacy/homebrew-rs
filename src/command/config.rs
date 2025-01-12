use crate::{brew, Config, Env};

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

/// 执行 `brew --cache` 命令
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let out = homebrew::cache().unwrap();
/// assert_eq!(out, "/Users/wxnacy/Library/Caches/Homebrew");
/// ```
pub fn cache() -> anyhow::Result<String> {
    brew("--cache")
}

/// 执行 `brew --version` 命令
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let out = homebrew::version().unwrap();
/// assert_eq!(out, "Homebrew 4.4.15-70-g1e91082");
/// ```
pub fn version() -> anyhow::Result<String> {
    brew("--version")
}

/// 执行 `brew --repository` 命令
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let out = homebrew::repository().unwrap();
/// assert_eq!(out, "/opt/homebrew");
/// ```
pub fn repository() -> anyhow::Result<String> {
    brew("--repository")
}

/// 执行 `brew --prefix` 命令
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let out = homebrew::prefix().unwrap();
/// assert_eq!(out, "/opt/homebrew");
/// ```
pub fn prefix() -> anyhow::Result<String> {
    brew("--prefix")
}

/// 执行 `brew --caskroom` 命令
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let out = homebrew::caskroom().unwrap();
/// assert_eq!(out, "/opt/homebrew/Caskroom");
/// ```
pub fn caskroom() -> anyhow::Result<String> {
    brew("--caskroom")
}

/// 执行 `brew --cellar` 命令
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let out = homebrew::cellar().unwrap();
/// assert_eq!(out, "/opt/homebrew/Cellar");
/// ```
pub fn cellar() -> anyhow::Result<String> {
    brew("--cellar")
}

/// 执行 `brew --env --plain` 命令并得到结构体
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let config = homebrew::env().unwrap();
/// assert_eq!(config.cmake_prefix_path, "/opt/homebrew");
/// assert_eq!(config.git, "git");
/// ```
pub fn env() -> anyhow::Result<Env> {
    let output = brew("--env --plain")?;
    Env::from(&output)
}

/// 执行 `brew --env --shell=auto` 命令并得到 shell 文本
///
/// Examples
///
/// ```
/// use homebrew;
///
/// let config = homebrew::env_shell().unwrap();
/// println!("{config}");
/// ```
pub fn env_shell() -> anyhow::Result<String> {
    brew("--env --shell=auto")
}

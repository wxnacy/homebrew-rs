use std::{collections::HashMap, ffi::OsStr, process::Command};

use anyhow::{anyhow, Result};

use crate::config::get_brew_bin;

/// `brew` 命令构造器
///
/// 有一个快速执行方式见 [`brew`]
///
/// Examples
///
/// ```
/// extern crate homebrew;
///
/// let out = homebrew::Brew::new("--caskroom")
///     .set_env("HOMEBREW_NO_AUTO_UPDATE", "1")
///     .run().unwrap();
///
/// let out2 = homebrew::brew("--caskroom").unwrap();
///
/// assert_eq!(out, out2);
/// ```
#[derive(Debug, Clone)]
pub struct Brew<T>
where
    T: AsRef<OsStr> + Clone + Eq + std::hash::Hash,
{
    cmd_: T,
    env_: HashMap<T, T>,
}
impl<T> Brew<T>
where
    T: AsRef<OsStr> + Clone + Eq + std::hash::Hash,
{
    /// 新建一个 `brew` 命令构造器
    pub fn new(cmd_: T) -> Self {
        let env_ = HashMap::new();
        Self { cmd_, env_ }
    }

    /// 添加环境变量
    pub fn set_env(&mut self, key: T, value: T) -> &mut Self {
        self.env_.insert(key, value);
        self
    }

    /// 运行构造的 `brew` 命令
    pub fn run(&self) -> Result<String> {
        let bin = get_brew_bin()?;
        let cmd_str = self.cmd_.as_ref().to_string_lossy();
        let cmds = cmd_str.split(' ');
        let output = Command::new(bin)
            .args(cmds)
            .envs(&self.env_)
            .output()?;

        let outerr = String::from_utf8_lossy(&output.stderr);
        if !outerr.is_empty() {
            if let Some(e) = outerr.strip_suffix("\n") {
                return Err(anyhow!("{e}"));
            } else {
                return Err(anyhow!("{outerr}"));
            }
        }
        let out = String::from_utf8_lossy(&output.stdout);
        if let Some(o) = out.strip_suffix("\n") {
            return Ok(o.to_string());
        }
        Ok(out.to_string())
    }
}

/// 执行 `brew` 命令
///
/// 默认添加环境变量 HOMEBREW_NO_AUTO_UPDATE=1
///
/// 如果想要更复杂的构造执行器，请使用 [`Brew`]
///
/// Examples
///
/// ```
/// extern crate homebrew;
///
/// let out = homebrew::brew("--prefix").unwrap();
///
/// // arm64 架构的可以运行通过，intel 芯片请使用 `/usr/local/homebrew`
/// assert_eq!(out, "/opt/homebrew");
/// ```
pub fn brew(cmd: &str) -> Result<String> {
    Brew::new(cmd)
        .set_env("HOMEBREW_NO_AUTO_UPDATE", "1")
        .run()
}

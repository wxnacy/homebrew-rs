use std::{collections::HashMap, process::Command};

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
///     // .set_env("HOMEBREW_NO_AUTO_UPDATE", "1")
///     // use set_env_no_auto_update replace
///     .set_env_no_auto_update()
///     .run().unwrap();
///
/// let out2 = homebrew::brew("--caskroom").unwrap();
///
/// assert_eq!(out, out2);
/// ```
#[derive(Debug, Clone)]
pub struct Brew {
    cmd_: String,
    env_: HashMap<String, String>,
}

impl Brew {
    /// 新建一个 `brew` 命令构造器
    pub fn new<T: AsRef<str>>(cmd: T) -> Self {
        Self {
            cmd_: cmd.as_ref().to_string(),
            env_: HashMap::new(),
        }
    }

    /// 添加环境变量
    pub fn set_env<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self{
        self.env_.insert(key.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// 添加环境变量 HOMEBREW_NO_AUTO_UPDATE=1
    pub fn set_env_no_auto_update(&mut self) -> &mut Self {
        self.set_env("HOMEBREW_NO_AUTO_UPDATE", "1")
    }

    /// 运行构造的 `brew` 命令
    pub fn run(&self) -> Result<String> {
        let bin = get_brew_bin()?;
        let cmds = self.cmd_.split(' ');
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

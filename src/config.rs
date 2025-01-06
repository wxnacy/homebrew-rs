use std::process::Command;

use anyhow::{Ok, Result};


const BREW_BIN_ARM64: &str = "/opt/homebrew/bin/brew";
const BREW_BIN_X86_64: &str = "/opt/homebrew/bin/brew";

pub fn get_brew_bin() -> Result<String>{
    let output = Command::new("uname")
        .arg("-m")
        .output()?;   // 执行命令并获取输出
    if let Some(out) = String::from_utf8_lossy(&output.stdout).strip_suffix("\n") {
        if out == "x86_64" {
            return Ok(String::from(BREW_BIN_X86_64));
        }
    }
    Ok(String::from(BREW_BIN_ARM64))
}


#[cfg(test)]
mod tests {
    use crate::config::BREW_BIN_ARM64;

    use super::get_brew_bin;

    #[test]
    fn test_get_bin() {
        if let Ok(cmd) = get_brew_bin() {
            assert_eq!(cmd, BREW_BIN_ARM64)
        }
    }
}

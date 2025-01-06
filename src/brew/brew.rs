use std::process::Command;

use anyhow::{anyhow, Result};

use crate::config::get_brew_bin;


pub fn brew(cmd: &str) -> Result<String> {
    let bin = get_brew_bin()?;
    let output = Command::new(bin)
        .args(cmd.split(" "))
        .env("HOMEBREW_NO_AUTO_UPDATE", "1")
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
        return Ok(o.to_string())
    }
    Ok(out.to_string())
}

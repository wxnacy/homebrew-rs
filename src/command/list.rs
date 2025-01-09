
use anyhow::Result;

use crate::Brew;

/// 列举出 `brew` 安装的包列表，包含 `Cask` 和 `Formulae`
pub fn list() -> Result<Vec<String>> {
    Brew::default()
        .set_cmd("list")
        .output_vec()
}

/// 列举出 `brew` 安装的包列表，只包含 `Cask`
pub fn list_cask() -> Result<Vec<String>> {
    Brew::default()
        .set_cmd("list --cask")
        .output_vec()
}

/// 列举出 `brew` 安装的包列表，只包含 `Formulae`
pub fn list_formulae() -> Result<Vec<String>> {
    Brew::default()
        .set_cmd("list --formula")
        .output_vec()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_list() -> anyhow::Result<()> {
        let all_pkgs = list()?;
        let cask_pkgs = list_cask()?;
        let formulae_pkgs = list_formulae()?;
        assert_eq!(all_pkgs.len(), cask_pkgs.len() + formulae_pkgs.len());
        Ok(())
    }
}

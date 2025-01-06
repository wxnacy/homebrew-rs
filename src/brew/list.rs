
use anyhow::Result;

use crate::brew::brew::brew;

/// 列举出 `brew` 安装的包列表，包含 `Cask` 和 `Formulae`
pub fn list() -> Result<Vec<String>> {
    let out = brew("list")?;
    let res: Vec<String> = out.split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    Ok(res)
}

/// 列举出 `brew` 安装的包列表，只包含 `Cask`
pub fn list_cask() -> Result<Vec<String>> {
    let out = brew("list --cask")?;
    let res: Vec<String> = out.split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    Ok(res)
}

/// 列举出 `brew` 安装的包列表，只包含 `Formulae`
pub fn list_formulae() -> Result<Vec<String>> {
    let out = brew("list --formulae")?;
    let res: Vec<String> = out.split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    Ok(res)
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

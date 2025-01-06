use serde::{Serialize, Deserialize};

use crate::{Cask, Formula};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "casks")]
    casks: Vec<Cask>,
    #[serde(rename = "formulae")]
    formula: Vec<Formula>,

    #[serde(skip)]
    pub name: String,
    #[serde(skip)]
    pub full_name: String,
    #[serde(skip)]
    pub tap: String,
    #[serde(skip)]
    pub desc: String,
    #[serde(skip)]
    pub homepage: String,
}

impl Package {
    /// 通过 `json` 字符串来新建
    pub fn from(json_str: &str) -> anyhow::Result<Self> {
        let mut pkg: Package = serde_json::from_str(json_str)?;
        // Clone values first before modifying
        let (name, full_name, tap, desc, homepage) = if pkg.is_cask() {
            let p = pkg.cask();
            (
                p.token.clone(),
                p.full_token.clone(),
                p.tap.clone(),
                p.desc.clone(),
                p.homepage.clone(),
            )
        } else {
            let p = pkg.formula();
            (
                p.name.clone(),
                p.full_name.clone(),
                p.tap.clone(),
                p.desc.clone(),
                p.homepage.clone(),
            )
        };

        pkg.name = name;
        pkg.full_name = full_name;
        pkg.tap = tap;
        pkg.desc = desc;
        pkg.homepage = homepage;

        Ok(pkg)
    }
    pub fn is_cask(&self) -> bool {
        if !self.casks.is_empty() { true } else { false }
    }
    pub fn formula(&self) -> &Formula {
        &self.formula[0]
    }
    pub fn cask(&self) -> &Cask {
        &self.casks[0]
    }
}

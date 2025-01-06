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

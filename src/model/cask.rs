use serde::{Serialize, Deserialize};
use std::{fmt};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Cask {
    pub token: String,
    pub full_token: String,
    pub old_tokens: Vec<String>,
    pub tap: String,
    pub name: Vec<String>,
    pub desc: Option<String>,
    pub homepage: String,
    pub url: String,
    pub url_specs: UrlSpecs,
    pub version: String,
    pub installed: Option<String>,
    pub installed_time: Option<u64>,
    pub bundle_version: Option<String>,
    pub bundle_short_version: Option<String>,
    pub outdated: bool,
    pub sha256: Option<String>,
    pub artifacts: Vec<Artifact>,
    pub caveats: Option<String>,
    pub depends_on: DependsOn,
    pub conflicts_with: Option<ConflictsWith>,
    pub container: Option<Container>,
    pub auto_updates: Option<AutoUpdates>,
    pub deprecated: bool,
    pub deprecation_date: Option<String>,
    pub deprecation_reason: Option<String>,
    pub deprecation_replacement: Option<String>,
    pub disabled: bool,
    pub disable_date: Option<String>,
    pub disable_reason: Option<String>,
    pub disable_replacement: Option<String>,
    pub tap_git_head: Option<String>,
    pub languages: Vec<String>,
    pub ruby_source_path: Option<String>,
    pub ruby_source_checksum: Option<RubySourceChecksum>,
}

impl Cask {
    /// 判断 `Cask` 包是否已经安装
    pub fn is_installed(&self) -> bool {
        self.installed.is_some()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Container {
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // 允许根据 JSON 的结构选择合适的变体
pub enum AutoUpdates {
    Bool(bool),
    String(String),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UrlSpecs {
    // Add fields here if any exist in the original JSON
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Artifact {
    pub preflight: Option<String>,
    pub app: Option<Vec<App>>,
    pub uninstall: Option<Vec<Uninstall>>,
    pub installer: Option<Vec<Uninstall>>,
    pub binary: Option<Vec<Binary>>,
    pub zap: Option<Vec<Zap>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Installer {
    pub script: Option<InstallerScript>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct InstallerScript {
    pub args: Option<Vec<String>>,
    pub executable: Option<String>,
    pub sudo: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // 允许根据 JSON 的结构选择合适的变体
pub enum Uninstall {
    Struct(UninstallStruct),
    String(String),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UninstallStruct {
    pub quit: Option<Quit>,
    pub delete: Option<Quit>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // 允许根据 JSON 的结构选择合适的变体
pub enum Quit {
    Vec(Vec<String>),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // 允许根据 JSON 的结构选择合适的变体
pub enum App {
    Struct(AppStruct),
    String(String),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppStruct {
    pub target: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // 允许根据 JSON 的结构选择合适的变体
pub enum Binary {
    Struct(BinaryStruct),
    String(String),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BinaryStruct {
    pub target: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Zap {
    pub trash: Option<Trash>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // 允许根据 JSON 的结构选择合适的变体
pub enum Trash {
    Vec(Vec<String>),
    String(String),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DependsOn {
    pub macos: Option<MacOS>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MacOS {
    #[serde(rename = ">=")]
    pub version_gte: Option<Vec<String>>,
    #[serde(rename = "==")]
    pub version_eq: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ConflictsWith {
    pub cask: Option<Vec<String>>,
    pub formula: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RubySourceChecksum {
    pub sha256: String,
}

impl fmt::Display for UrlSpecs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{...}}")
    }
}

// impl fmt::Display for Artifact {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let mut parts = Vec::new();
        // if let Some(ref pf) = self.preflight {
            // parts.push(format!("preflight: {}", pf));
        // }
        // if let Some(ref apps) = self.app {
            // parts.push(format!("apps: [{}]", apps.join(", ")));
        // }
        // if let Some(ref bins) = self.binary {
            // parts.push(format!("binaries: [{}]", bins.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ")));
        // }
        // if let Some(ref z) = self.zap {
            // parts.push(format!("zap: {}", z));
        // }
        // write!(f, "{{{}}}", parts.join(", "))
    // }
// }

// impl fmt::Display for Binary {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}=>{}", self.target, self.path)
    // }
// }

// impl fmt::Display for Zap {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "[{}]", self.trash.join(", "))
    // }
// }

// impl fmt::Display for DependsOn {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.macos)
    // }
// }

// impl fmt::Display for MacOS {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "macOS >= {}", self.version_gte.join(" || "))
    // }
// }

// impl fmt::Display for ConflictsWith {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "conflicts with: [{}]", self.cask.join(", "))
    // }
// }

impl fmt::Display for RubySourceChecksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sha256)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, fs::{self, File}, path::PathBuf};
    use crate::brew::brew;
    use serde_json::{self, Value};

    const JSON_DATA: &str = r#"
    {
        "token": "kitty",
        "full_token": "kitty",
        "old_tokens": [],
        "tap": "homebrew/cask",
        "name": ["kitty"],
        "desc": "GPU-based terminal emulator",
        "homepage": "",
        "url": "",
        "url_specs": {},
        "version": "0.38.1",
        "installed": "0.38.1",
        "installed_time": 1735519389,
        "bundle_version": "0.38.1",
        "bundle_short_version": "0.38.1",
        "outdated": false,
        "sha256": "2971db2c2220a9f353efcc1d58d8b88462dc5a2a992adea9c051bf6e54c90e52",
        "artifacts": [
            {
                "preflight": null
            },
            {
                "app": ["kitty.app"]
            },
            {
                "binary": [
                    {
                        "path": "/opt/homebrew/Caskroom/kitty/0.38.1/kitty.wrapper.sh",
                        "target": "kitty"
                    }
                ]
            },
            {
                "binary": [
                    {
                        "path": "/opt/homebrew/Caskroom/kitty/0.38.1/kitten.wrapper.sh",
                        "target": "kitten"
                    }
                ]
            },
            {
                "zap": {
                    "trash": [
                        "~/.config/kitty",
                        "~/Library/Caches/kitty",
                        "~/Library/Preferences/kitty",
                        "~/Library/Preferences/net.kovidgoyal.kitty.plist",
                        "~/Library/Saved Application State/net.kovidgoyal.kitty.savedState"
                    ]
                }
            }
        ],
        "caveats": null,
        "depends_on": {
            "macos": {
                ">=": ["11"]
            }
        },
        "conflicts_with": {
            "cask": ["kitty@nightly"]
        },
        "container": null,
        "auto_updates": null,
        "deprecated": false,
        "deprecation_date": null,
        "deprecation_reason": null,
        "deprecation_replacement": null,
        "disabled": false,
        "disable_date": null,
        "disable_reason": null,
        "disable_replacement": null,
        "tap_git_head": "caacd9e842f1e843d1b31c629652c6dc0005e4bd",
        "languages": [],
        "ruby_source_path": "Casks/k/kitty.rb",
        "ruby_source_checksum": {
            "sha256": "1107b16dc344bace347af11c9b983456f0634cb53103beddbcda6413a10cce4a"
        }
    }
    "#;

    #[test]
    #[ignore]
    fn test_json_to_cask() {
        let package: Cask = serde_json::from_str(JSON_DATA).expect("Failed to parse JSON");

        // 进行一些断言以验证解析结果
        assert_eq!(package.token, "kitty");
        assert_eq!(package.version, "0.38.1");
        assert!(package.old_tokens.is_empty());
        assert_eq!(package.artifacts.len(), 5);
    }

    #[test]
    fn test_all_to_cask() {
        let tempfile = PathBuf::from("target/package.json");
        if !tempfile.exists() {
            let out = brew::brew("info --eval-all --json=v2").expect("Failed get info all");
            let file = File::create("package.json").expect("Failed to create file");
            let value: HashMap<String, Value> = serde_json::from_str(&out).expect("");
            let writer = std::io::BufWriter::new(file);
            serde_json::to_writer(writer, &value).expect("Failed to write package.json");
        }

        let out = fs::read_to_string(&tempfile).expect("Failed read package.json");
        let all_packages: HashMap<String, Value> = serde_json::from_str(&out).expect("Failed parse json string");

        for (key, value) in &all_packages {
            println!("{key}");
            if key == "formulae" {
                continue;
            }
            let packages: Vec<Value> = value.as_array().expect("Faield to as array").to_vec();
            for val in &packages {
                let name = &val["token"];
                let text = serde_json::to_string_pretty(val).expect("Faield to string");
                let pkg_m = serde_json::from_value::<Cask>(val.clone()).map_err(|e| anyhow::Error::new(e));
                match pkg_m{
                    Ok(pkg) => assert_eq!(&pkg.token, name),
                    Err(e) => {
                        fs::write("target/cask.json", &text).expect("Failed write");
                        println!("{}\n{}", text, name);
                        panic!("{e}")
                    }
                }
            }
            println!("{}", packages.len());
        }
    }

}

use serde::{Serialize, Deserialize};
use std::{fmt};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Cask {
    pub token: String,
    pub full_token: String,
    pub old_tokens: Vec<String>,
    pub tap: String,
    pub name: Vec<String>,
    pub desc: String,
    pub homepage: String,
    pub url: String,
    pub url_specs: UrlSpecs,
    pub version: String,
    pub installed: String,
    pub installed_time: i64,
    pub bundle_version: String,
    pub bundle_short_version: String,
    pub outdated: bool,
    pub sha256: String,
    pub artifacts: Vec<Artifact>,
    pub caveats: Option<String>,
    pub depends_on: DependsOn,
    pub conflicts_with: ConflictsWith,
    pub container: Option<String>,
    pub auto_updates: Option<String>,
    pub deprecated: bool,
    pub deprecation_date: Option<String>,
    pub deprecation_reason: Option<String>,
    pub deprecation_replacement: Option<String>,
    pub disabled: bool,
    pub disable_date: Option<String>,
    pub disable_reason: Option<String>,
    pub disable_replacement: Option<String>,
    pub tap_git_head: String,
    pub languages: Vec<String>,
    pub ruby_source_path: String,
    pub ruby_source_checksum: RubySourceChecksum,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UrlSpecs {
    // Add fields here if any exist in the original JSON
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Artifact {
    pub preflight: Option<String>,
    pub app: Option<Vec<String>>,
    pub binary: Option<Vec<Binary>>,
    pub zap: Option<Zap>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Binary {
    pub target: String,
    pub path: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Zap {
    pub trash: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DependsOn {
    pub macos: MacOS,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MacOS {
    #[serde(rename = ">=")]
    pub version_gte: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ConflictsWith {
    pub cask: Vec<String>,
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

impl fmt::Display for Artifact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = Vec::new();
        if let Some(ref pf) = self.preflight {
            parts.push(format!("preflight: {}", pf));
        }
        if let Some(ref apps) = self.app {
            parts.push(format!("apps: [{}]", apps.join(", ")));
        }
        if let Some(ref bins) = self.binary {
            parts.push(format!("binaries: [{}]", bins.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ")));
        }
        if let Some(ref z) = self.zap {
            parts.push(format!("zap: {}", z));
        }
        write!(f, "{{{}}}", parts.join(", "))
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}=>{}", self.target, self.path)
    }
}

impl fmt::Display for Zap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.trash.join(", "))
    }
}

impl fmt::Display for DependsOn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.macos)
    }
}

impl fmt::Display for MacOS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "macOS >= {}", self.version_gte.join(" || "))
    }
}

impl fmt::Display for ConflictsWith {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conflicts with: [{}]", self.cask.join(", "))
    }
}

impl fmt::Display for RubySourceChecksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sha256)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
    fn test_json_to_cask() {
        let package: Cask = serde_json::from_str(JSON_DATA).expect("Failed to parse JSON");

        // 进行一些断言以验证解析结果
        assert_eq!(package.token, "kitty");
        assert_eq!(package.version, "0.38.1");
        assert!(package.old_tokens.is_empty());
        assert_eq!(package.artifacts.len(), 5);
        assert_eq!(package.depends_on.macos.version_gte[0], "11");
    }
}

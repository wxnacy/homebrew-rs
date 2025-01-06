use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Formula {
    pub name: String,
    pub full_name: String,
    pub tap: String,
    pub oldnames: Vec<String>,
    pub aliases: Vec<String>,
    pub versioned_formulae: Vec<String>,
    pub desc: String,
    pub license: String,
    pub homepage: String,
    pub versions: Versions,
    pub urls: Urls,
    pub revision: i32,
    pub version_scheme: i32,
    pub bottle: Bottle,
    pub pour_bottle_only_if: Option<String>,
    pub keg_only: bool,
    pub keg_only_reason: Option<String>,
    pub options: Vec<String>,
    pub build_dependencies: Vec<String>,
    pub dependencies: Vec<String>,
    pub test_dependencies: Vec<String>,
    pub recommended_dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub uses_from_macos: Vec<String>,
    pub uses_from_macos_bounds: Vec<UsesFromMacOsBounds>,
    pub requirements: Vec<String>,
    pub conflicts_with: Vec<String>,
    pub conflicts_with_reasons: Vec<String>,
    pub link_overwrite: Vec<String>,
    pub caveats: Option<String>,
    pub installed: Vec<Installed>,
    pub linked_keg: String,
    pub pinned: bool,
    pub outdated: bool,
    pub deprecated: bool,
    pub deprecation_date: Option<String>,
    pub deprecation_reason: Option<String>,
    pub deprecation_replacement: Option<String>,
    pub disabled: bool,
    pub disable_date: Option<String>,
    pub disable_reason: Option<String>,
    pub disable_replacement: Option<String>,
    pub post_install_defined: bool,
    pub service: Option<String>,
    pub tap_git_head: String,
    pub ruby_source_path: String,
    pub ruby_source_checksum: RubySourceChecksum,
    pub head_dependencies: Option<HeadDependencies>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Versions {
    pub stable: String,
    pub head: String,
    pub bottle: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Urls {
    pub stable: Url,
    pub head: Url,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Url {
    pub url: String,
    pub tag: Option<String>,
    pub revision: Option<String>,
    pub using: Option<String>,
    pub checksum: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Bottle {
    pub stable: StableBottle,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StableBottle {
    pub rebuild: i32,
    pub root_url: String,
    pub files: std::collections::HashMap<String, BottleFile>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BottleFile {
    pub cellar: String,
    pub url: String,
    pub sha256: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UsesFromMacOsBounds {
    // Add fields here if any exist in the original JSON
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Installed {
    pub version: String,
    pub used_options: Vec<String>,
    pub built_as_bottle: bool,
    pub poured_from_bottle: bool,
    pub time: i64,
    pub runtime_dependencies: Vec<RuntimeDependency>,
    pub installed_as_dependency: bool,
    pub installed_on_request: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RuntimeDependency {
    pub full_name: String,
    pub version: String,
    pub revision: i32,
    pub pkg_version: String,
    pub declared_directly: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RubySourceChecksum {
    pub sha256: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct HeadDependencies {
    pub build_dependencies: Vec<String>,
    pub dependencies: Vec<String>,
    pub test_dependencies: Vec<String>,
    pub recommended_dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub uses_from_macos: Vec<String>,
    pub uses_from_macos_bounds: Vec<UsesFromMacOsBounds>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    const JSON_DATA: &str = r#"
    {
      "name": "wget",
      "full_name": "wget",
      "tap": "homebrew/core",
      "oldnames": [],
      "aliases": [],
      "versioned_formulae": [],
      "desc": "Internet file retriever",
      "license": "GPL-3.0-or-later",
      "homepage": "https://www.gnu.org/software/wget/",
      "versions": {
        "stable": "1.25.0",
        "head": "HEAD",
        "bottle": true
      },
      "urls": {
        "stable": {
          "url": "https://ftp.gnu.org/gnu/wget/wget-1.25.0.tar.gz",
          "tag": null,
          "revision": null,
          "using": null,
          "checksum": "766e48423e79359ea31e41db9e5c289675947a7fcf2efdcedb726ac9d0da3784"
        },
        "head": {
          "url": "https://git.savannah.gnu.org/git/wget.git",
          "branch": "master",
          "using": null
        }
      },
      "revision": 0,
      "version_scheme": 0,
      "bottle": {
        "stable": {
          "rebuild": 0,
          "root_url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles",
          "files": {
            "arm64_sequoia": {
              "cellar": "/opt/homebrew/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.arm64_sequoia.bottle.tar.gz",
              "sha256": "a93dd95c5d63036e026b526e000d33fae7fb44d9a8fda5afc89bff112438c6b3"
            },
            "arm64_sonoma": {
              "cellar": "/opt/homebrew/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.arm64_sonoma.bottle.tar.gz",
              "sha256": "4d180cd4ead91a34e2c2672189fc366b87ae86e6caa3acbf4845b272f57c859a"
            },
            "arm64_ventura": {
              "cellar": "/opt/homebrew/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.arm64_ventura.bottle.tar.gz",
              "sha256": "7fce09705a52a2aff61c4bdd81b9d2a1a110539718ded2ad45562254ef0f5c22"
            },
            "sonoma": {
              "cellar": "/usr/local/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.sonoma.bottle.tar.gz",
              "sha256": "5650778a8e7a60c2dea9412dd21d2f5e8ff4f224dbefbdf54924b99012062edc"
            },
            "ventura": {
              "cellar": "/usr/local/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.ventura.bottle.tar.gz",
              "sha256": "78cee523a9b58a7b824b51767935f68c9838e9f673e70d001982858001e766ff"
            },
            "x86_64_linux": {
              "cellar": "/home/linuxbrew/.linuxbrew/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.x86_64_linux.bottle.tar.gz",
              "sha256": "ab5f3c1c60bef4e2a4781e9b29af8afb48ead837136c419edd7febdf44b59058"
            }
          }
        }
      },
      "pour_bottle_only_if": null,
      "keg_only": false,
      "keg_only_reason": null,
      "options": [],
      "build_dependencies": [
        "pkgconf"
      ],
      "dependencies": [
        "libidn2",
        "openssl@3",
        "gettext",
        "libunistring"
      ],
      "test_dependencies": [],
      "recommended_dependencies": [],
      "optional_dependencies": [],
      "uses_from_macos": [
        "zlib"
      ],
      "uses_from_macos_bounds": [
        {}
      ],
      "requirements": [],
      "conflicts_with": [],
      "conflicts_with_reasons": [],
      "link_overwrite": [],
      "caveats": null,
      "installed": [
        {
          "version": "1.25.0",
          "used_options": [],
          "built_as_bottle": true,
          "poured_from_bottle": true,
          "time": 1734795904,
          "runtime_dependencies": [
            {
              "full_name": "libunistring",
              "version": "1.3",
              "revision": 0,
              "pkg_version": "1.3",
              "declared_directly": true
            },
            {
              "full_name": "gettext",
              "version": "0.23",
              "revision": 0,
              "pkg_version": "0.23",
              "declared_directly": true
            },
            {
              "full_name": "libidn2",
              "version": "2.3.7",
              "revision": 0,
              "pkg_version": "2.3.7",
              "declared_directly": true
            },
            {
              "full_name": "ca-certificates",
              "version": "2024-11-26",
              "revision": 0,
              "pkg_version": "2024-11-26",
              "declared_directly": false
            },
            {
              "full_name": "openssl@3",
              "version": "3.4.0",
              "revision": 0,
              "pkg_version": "3.4.0",
              "declared_directly": true
            }
          ],
          "installed_as_dependency": false,
          "installed_on_request": true
        }
      ],
      "linked_keg": "1.25.0",
      "pinned": false,
      "outdated": false,
      "deprecated": false,
      "deprecation_date": null,
      "deprecation_reason": null,
      "deprecation_replacement": null,
      "disabled": false,
      "disable_date": null,
      "disable_reason": null,
      "disable_replacement": null,
      "post_install_defined": false,
      "service": null,
      "tap_git_head": "5269812199d74ce7e80f9893b99f84ba4dd7c344",
      "ruby_source_path": "Formula/w/wget.rb",
      "ruby_source_checksum": {
        "sha256": "3dec91401db7bff6591a5065a21859565b8c9588a6baf1b1aa56692775df11ff"
      },
      "head_dependencies": {
        "build_dependencies": [
          "autoconf",
          "automake",
          "xz",
          "pkgconf"
        ],
        "dependencies": [
          "libidn2",
          "openssl@3",
          "gettext",
          "libunistring"
        ],
        "test_dependencies": [],
        "recommended_dependencies": [],
        "optional_dependencies": [],
        "uses_from_macos": [
          "zlib"
        ],
        "uses_from_macos_bounds": [
          {}
        ]
      }
    }
    "#;

    #[test]
    fn test_json_to_formulae() {
        let package: Formula = serde_json::from_str(JSON_DATA).expect("Failed to parse JSON");

        // 进行一些断言以验证解析结果
        assert_eq!(package.name, "wget");
        assert_eq!(package.versions.stable, "1.25.0");
        assert!(package.oldnames.is_empty());
        assert!(package.dependencies.contains(&"libidn2".to_string()));
        assert!(package.installed.len() > 0);
        assert_eq!(package.bottle.stable.files.len(), 6);
    }
}

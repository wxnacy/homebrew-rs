use serde::{Serialize, Deserialize};

/// `Formula` 包的结构体
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Formula {
    pub name: String,
    pub full_name: String,
    pub tap: String,
    pub oldnames: Vec<String>,
    pub aliases: Vec<String>,
    pub versioned_formulae: Vec<String>,
    pub desc: String,
    pub license: Option<String>,
    pub homepage: String,
    pub versions: Versions,
    pub urls: Urls,
    pub revision: i32,
    pub version_scheme: i32,
    pub bottle: Bottle,
    pub pour_bottle_only_if: Option<String>,
    pub keg_only: bool,
    pub keg_only_reason: Option<KegOnlyReason>,
    pub options: Vec<Options>,
    pub build_dependencies: Vec<String>,
    pub dependencies: Vec<String>,
    pub test_dependencies: Vec<String>,
    pub recommended_dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub uses_from_macos: Vec<UsesFromMacOs>,
    pub uses_from_macos_bounds: Vec<UsesFromMacOsBounds>,
    pub requirements: Vec<Requirement>,
    pub conflicts_with: Vec<String>,
    pub conflicts_with_reasons: Vec<Option<String>>,
    pub link_overwrite: Vec<String>,
    pub caveats: Option<String>,
    pub installed: Vec<Installed>,
    pub linked_keg: Option<String>,
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
    pub service: Option<Service>,
    pub tap_git_head: Option<String>,
    pub ruby_source_path: Option<String>,
    pub ruby_source_checksum: RubySourceChecksum,
    pub head_dependencies: Option<HeadDependencies>,
}

impl Formula {
    /// 判断 `Formula` 包是否已安装
    pub fn is_installed(&self) -> bool {
        !self.installed.is_empty()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Options {
    option: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Service {
    run: Option<ServiceRun>,         // run 是一个字符串数组
    run_type: Option<String>,        // run_type 是字符串
    working_dir: Option<String>,     // working_dir 是字符串
    error_log_path: Option<String>, // error_log_path 是字符串
    keep_alive: Option<KeepAlive>,   // keep_alive 是 KeepAlive 结构体
    log_path: Option<String>,        // log_path 是字符串
    environment_variables: Option<EnvironmentVariables>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeepAlive {
    always: Option<bool>,
    successful_exit: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KegOnlyReason {
    pub explanation: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Requirement {
    cask: Option<String>,           // cask 可以是 null
    contexts: Vec<String>,          // contexts 是一个字符串数组
    download: Option<String>,       // download 可以是 null
    name: String,                   // name 是字符串
    specs: Vec<String>,             // specs 是一个字符串数组
    version: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Versions {
    pub stable: Option<String>,
    pub head: Option<String>,
    pub bottle: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Urls {
    pub stable: Option<Url>,
    pub head: Option<Url>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Url {
    pub url: String,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub revision: Option<String>,
    pub using: Option<String>,
    pub checksum: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Bottle {
    pub stable: Option<StableBottle>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StableBottle {
    pub rebuild: i32,
    pub root_url: Option<String>,
    pub files: std::collections::HashMap<String, BottleFile>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BottleFile {
    pub cellar: String,
    pub url: String,
    pub sha256: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // 允许根据 JSON 的结构选择合适的变体
pub enum ServiceRun {
    String(String),
    Vec(Vec<String>),
    Struct(ServiceRunStruct),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ServiceRunStruct {
    pub macos: Option<Vec<String>>,
    pub linux: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct EnvironmentVariables {
    #[serde(rename = "PATH")]
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // 允许根据 JSON 的结构选择合适的变体
pub enum UsesFromMacOs {
    String(String),
    Struct(UsesFromMacOsStruct)
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UsesFromMacOsStruct {
    pub bison: Option<String>,
    pub flex: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UsesFromMacOsBounds {
    // Add fields here if any exist in the original JSON
    pub since: Option<String>,
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
    pub uses_from_macos: Vec<UsesFromMacOs>,
    pub uses_from_macos_bounds: Vec<UsesFromMacOsBounds>,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::{self, File}, path::PathBuf};

    use crate::brew::brew;

    use super::*;
    use serde_json::{self, Value};

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
        // assert_eq!(package.versions.stable, "1.25.0");
        assert!(package.oldnames.is_empty());
        assert!(package.dependencies.contains(&"libidn2".to_string()));
        assert!(package.installed.len() > 0);
        // assert_eq!(package.bottle.stable.files.len(), 6);
    }

    #[test]
    fn test_all_to_formulae() {
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
            if key == "casks" {
                continue;
            }
            let packages: Vec<Value> = value.as_array().expect("Faield to as array").to_vec();
            for val in &packages {
                let name = &val["name"];
                let text = serde_json::to_string_pretty(val).expect("Faield to string");
                let pkg_m = serde_json::from_value::<Formula>(val.clone()).map_err(|e| anyhow::Error::new(e));
                match pkg_m{
                    Ok(pkg) => assert_eq!(&pkg.name, name),
                    Err(e) => {
                        fs::write("target/formula.json", &text).expect("Failed write");
                        println!("{}\n{}", text, name);
                        panic!("{e}")
                    }
                }
            }
            println!("{}", packages.len());
        }
    }

}

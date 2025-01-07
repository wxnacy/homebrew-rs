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
    use std::{collections::HashMap, fs, path::PathBuf};

    use crate::info_all;

    use super::*;
    use serde_json::{self, Value};

    #[test]
    fn test_all_to_formulae() {
        let tempfile = PathBuf::from("target/package.json");
        if !tempfile.exists() {
            let pkg = info_all().unwrap();
            pkg.to_file(&tempfile).unwrap();
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

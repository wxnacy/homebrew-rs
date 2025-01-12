use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// `Config` 的结构体
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "HOMEBREW_VERSION")]
    pub version: String,

    #[serde(rename = "ORIGIN")]
    pub origin: String,

    #[serde(rename = "HEAD")]
    pub head: String,

    #[serde(rename = "Last commit")]
    pub last_commit: String,

    #[serde(rename = "Branch")]
    pub branch: String,

    #[serde(rename = "Core tap JSON")]
    pub core_tap_json: String,

    #[serde(rename = "Core cask tap JSON")]
    pub core_cask_tap_json: String,

    #[serde(rename = "HOMEBREW_PREFIX")]
    pub prefix: String,

    #[serde(rename = "HOMEBREW_API_DOMAIN")]
    pub api_domain: String,

    #[serde(rename = "HOMEBREW_BOTTLE_DOMAIN")]
    pub bottle_domain: String,

    #[serde(rename = "HOMEBREW_BREW_GIT_REMOTE")]
    pub brew_git_remote: String,

    #[serde(rename = "HOMEBREW_CASK_OPTS", deserialize_with = "deserialize_vec")]
    pub cask_opts: Vec<String>,

    #[serde(rename = "HOMEBREW_CORE_GIT_REMOTE")]
    pub core_git_remote: String,

    #[serde(rename = "HOMEBREW_EDITOR")]
    pub editor: String,

    #[serde(rename = "HOMEBREW_MAKE_JOBS", deserialize_with = "deserialize_u32")]
    pub make_jobs: u32,

    #[serde(rename = "HOMEBREW_PIP_INDEX_URL")]
    pub pip_index_url: String,

    #[serde(rename = "HOMEBREW_SORBET_RUNTIME")]
    pub sorbet_runtime: String,

    #[serde(rename = "Homebrew Ruby")]
    pub ruby: String,

    #[serde(rename = "CPU")]
    pub cpu: String,

    #[serde(rename = "Clang")]
    pub clang: String,

    #[serde(rename = "Git")]
    pub git: String,

    #[serde(rename = "Curl")]
    pub curl: String,

    #[serde(rename = "macOS")]
    pub macos: String,

    #[serde(rename = "CLT")]
    pub clt: String,

    #[serde(rename = "Xcode")]
    pub xcode: String,

    #[serde(rename = "Rosetta 2", deserialize_with = "deserialize_bool")]
    pub rosetta2: bool,
}


/// TODO: add display fn
impl Config {
    ///
    ///
    /// Examples
    ///
    /// ```
    /// use homebrew;
    ///
    /// let text = r#"
    /// HOMEBREW_VERSION: 4.4.15-56-g6aac197
    /// ORIGIN: https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git
    /// HEAD: 6aac197d556f60e82490dcb46fcbe7090c8934e9
    /// Last commit: 2 days ago
    /// Branch: master
    /// Core tap JSON: 10 Jan 14:49 UTC
    /// Core cask tap JSON: 10 Jan 14:49 UTC
    /// HOMEBREW_PREFIX: /opt/homebrew
    /// HOMEBREW_API_DOMAIN: https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/api
    /// HOMEBREW_BOTTLE_DOMAIN: https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles
    /// HOMEBREW_BREW_GIT_REMOTE: https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git
    /// HOMEBREW_CASK_OPTS: []
    /// HOMEBREW_CORE_GIT_REMOTE: https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/homebrew-core.git
    /// HOMEBREW_EDITOR: nvim
    /// HOMEBREW_MAKE_JOBS: 12
    /// HOMEBREW_PIP_INDEX_URL: https://pypi.tuna.tsinghua.edu.cn/simple
    /// HOMEBREW_SORBET_RUNTIME: set
    /// Homebrew Ruby: 3.3.6 => /opt/homebrew/Library/Homebrew/vendor/portable-ruby/3.3.6/bin/ruby
    /// CPU: dodeca-core 64-bit arm_brava
    /// Clang: 16.0.0 build 1600
    /// Git: 2.47.1 => /opt/homebrew/bin/git
    /// Curl: 8.7.1 => /usr/bin/curl
    /// macOS: 15.1-arm64
    /// CLT: 16.2.0.0.1.1733547573
    /// Xcode: N/A
    /// Rosetta 2: false
    /// "#;
    ///
    /// let config = homebrew::Config::from(text).unwrap();
    ///
    /// assert_eq!(config.version, "4.4.15-56-g6aac197");
    /// assert_eq!(config.origin, "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git");
    /// assert_eq!(config.last_commit, "2 days ago");
    /// assert_eq!(config.head, "6aac197d556f60e82490dcb46fcbe7090c8934e9");
    /// assert_eq!(config.branch, "master");
    /// assert_eq!(config.core_tap_json, "10 Jan 14:49 UTC");
    /// assert_eq!(config.core_cask_tap_json, "10 Jan 14:49 UTC");
    /// assert_eq!(config.prefix, "/opt/homebrew");
    /// assert_eq!(config.api_domain, "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/api");
    /// assert_eq!(config.bottle_domain, "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles");
    /// assert_eq!(config.brew_git_remote, "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git");
    /// assert_eq!(config.cask_opts, Vec::<String>::new());
    /// assert_eq!(config.core_git_remote, "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/homebrew-core.git");
    /// assert_eq!(config.editor, "nvim");
    /// assert_eq!(config.make_jobs, 12);
    /// assert_eq!(config.pip_index_url, "https://pypi.tuna.tsinghua.edu.cn/simple");
    /// assert_eq!(config.sorbet_runtime, "set");
    /// assert_eq!(config.ruby, "3.3.6 => /opt/homebrew/Library/Homebrew/vendor/portable-ruby/3.3.6/bin/ruby");
    /// assert_eq!(config.cpu, "dodeca-core 64-bit arm_brava");
    /// assert_eq!(config.clang, "16.0.0 build 1600");
    /// assert_eq!(config.git, "2.47.1 => /opt/homebrew/bin/git");
    /// assert_eq!(config.curl, "8.7.1 => /usr/bin/curl");
    /// assert_eq!(config.macos, "15.1-arm64");
    /// assert_eq!(config.clt, "16.2.0.0.1.1733547573");
    /// assert_eq!(config.xcode, "N/A");
    /// assert_eq!(config.rosetta2, false);
    /// ```
    pub fn from(text: &str) -> anyhow::Result<Self> {
        let text = text.strip_prefix("\n").unwrap_or(text);
        let text = text.strip_suffix("\n").unwrap_or(text);
        let mut data: HashMap<String, Value> = HashMap::new();
        for line in text.split("\n") {
            let kv: Vec<String> = line.split(": ").map(String::from).collect();
            data.insert(kv[0].clone(), Value::String(kv[1].clone()));
        }

        let json_str = serde_json::to_string(&data)?;
        let config: Self = serde_json::from_str(&json_str)?;

        Ok(config)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Env {
    #[serde(rename = "HOMEBREW_CC")]
    pub cc: String,

    #[serde(rename = "HOMEBREW_CXX")]
    pub cxx: String,

    #[serde(rename = "MAKEFLAGS")]
    pub makeflags: String,

    #[serde(rename = "CMAKE_PREFIX_PATH")]
    pub cmake_prefix_path: String,

    #[serde(rename = "CMAKE_INCLUDE_PATH")]
    pub cmake_include_path: String,

    #[serde(rename = "CMAKE_LIBRARY_PATH")]
    pub cmake_library_path: String,

    #[serde(rename = "PKG_CONFIG_LIBDIR")]
    pub pkg_config_libdir: String,

    #[serde(rename = "HOMEBREW_MAKE_JOBS", deserialize_with = "deserialize_u32")]
    pub make_jobs: u32,

    #[serde(rename = "HOMEBREW_GIT")]
    pub git: String,

    #[serde(rename = "HOMEBREW_SDKROOT")]
    pub sdkroot: String,

    #[serde(rename = "ACLOCAL_PATH")]
    pub aclocal_path: String,

    #[serde(rename = "PATH")]
    pub path: String,
}

impl Env {
    ///
    /// Creates an `Env` instance from a given string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use homebrew;
    ///
    /// let text = r#"
    /// HOMEBREW_CC: clang
    /// HOMEBREW_CXX: clang++
    /// MAKEFLAGS: -j12
    /// CMAKE_PREFIX_PATH: /opt/homebrew
    /// CMAKE_INCLUDE_PATH: /Library/Developer/CommandLineTools/SDKs/MacOSX15.sdk/System/Library/Frameworks/OpenGL.framework/Versions/Current/Headers
    /// CMAKE_LIBRARY_PATH: /Library/Developer/CommandLineTools/SDKs/MacOSX15.sdk/System/Library/Frameworks/OpenGL.framework/Versions/Current/Libraries
    /// PKG_CONFIG_LIBDIR: /usr/lib/pkgconfig:/opt/homebrew/Library/Homebrew/os/mac/pkgconfig/15
    /// HOMEBREW_MAKE_JOBS: 12
    /// HOMEBREW_GIT: git
    /// HOMEBREW_SDKROOT: /Library/Developer/CommandLineTools/SDKs/MacOSX15.sdk
    /// ACLOCAL_PATH: /opt/homebrew/share/aclocal
    /// PATH: /opt/homebrew/Library/Homebrew/shims/mac/super:/usr/bin:/bin:/usr/sbin:/sbin
    /// "#;
    ///
    /// let env = homebrew::Env::from(text).unwrap();
    ///
    /// assert_eq!(env.cc, "clang");
    /// assert_eq!(env.cxx, "clang++");
    /// assert_eq!(env.makeflags, "-j12");
    /// assert_eq!(env.cmake_prefix_path, "/opt/homebrew");
    /// assert_eq!(env.cmake_include_path, "/Library/Developer/CommandLineTools/SDKs/MacOSX15.sdk/System/Library/Frameworks/OpenGL.framework/Versions/Current/Headers");
    /// assert_eq!(env.cmake_library_path, "/Library/Developer/CommandLineTools/SDKs/MacOSX15.sdk/System/Library/Frameworks/OpenGL.framework/Versions/Current/Libraries");
    /// assert_eq!(env.pkg_config_libdir, "/usr/lib/pkgconfig:/opt/homebrew/Library/Homebrew/os/mac/pkgconfig/15");
    /// assert_eq!(env.make_jobs, 12);
    /// assert_eq!(env.git, "git");
    /// assert_eq!(env.sdkroot, "/Library/Developer/CommandLineTools/SDKs/MacOSX15.sdk");
    /// assert_eq!(env.aclocal_path, "/opt/homebrew/share/aclocal");
    /// assert_eq!(env.path, "/opt/homebrew/Library/Homebrew/shims/mac/super:/usr/bin:/bin:/usr/sbin:/sbin");
    /// ```
    pub fn from(text: &str) -> anyhow::Result<Self> {
        let text = text.strip_prefix("\n").unwrap_or(text);
        let text = text.strip_suffix("\n").unwrap_or(text);
        let mut data: HashMap<String, Value> = HashMap::new();
        for line in text.split("\n") {
            let kv: Vec<String> = line.split(": ").map(String::from).collect();
            data.insert(kv[0].clone(), Value::String(kv[1].clone()));
        }

        let env: Self = serde_json::from_value(serde_json::to_value(&data)?)?;

        Ok(env)
    }
}


/// 反序列化 u32 类型
fn deserialize_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

/// 反序列化 bool 类型
fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::custom("invalid boolean value")),
    }
}

/// 反序列化 Vec<String> 类型
fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s == "[]" {
        Ok(Vec::new())
    } else {
        Err(serde::de::Error::custom("invalid vector value"))
    }
}

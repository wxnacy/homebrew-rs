use std::{collections::HashMap, fs::File, io, path::Path};

use serde::{Serialize, Deserialize};
use serde_json::{Value, Result};

use crate::{Cask, Formula};

/// 运行 `brew info [name] --json=v2` 命令 `json` 结果反序列的结构体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "casks")]
    casks: Vec<Cask>,
    #[serde(rename = "formulae")]
    formulae: Vec<Formula>,

    /// `brew info [name] --json=v2` 命令原始数据
    #[serde(skip)]
    pub value: HashMap<String, Value>,

    /// = `self.formula().name`
    /// = `self.cask().token`
    #[serde(skip)]
    pub name: String,

    /// = `self.formula().full_name`
    /// = `self.cask().full_token`
    #[serde(skip)]
    pub full_name: String,

    /// = `self.formula().tap`
    /// = `self.cask().tap`
    #[serde(skip)]
    pub tap: String,

    /// = `self.formula().desc`
    /// = `Some(self.cask().desc)`
    #[serde(skip)]
    pub desc: String,

    /// = `self.formula().homepage`
    /// = `self.cask().homepage`
    #[serde(skip)]
    pub homepage: String,
}

impl Package {
    /// 通过 `json` 字符串来新建结构体
    ///
    /// Examples
    ///
    /// ```
    /// extern crate homebrew as brew;
    ///
    /// use std::fs;
    ///
    /// use brew::Package;
    ///
    /// fn main() {
    ///     let name = "wget";
    ///     let file = format!("examples/data/{}.json", name);
    ///     let json_str = fs::read_to_string(file).unwrap();
    ///     let pkg = Package::from(&json_str).unwrap();
    ///
    ///     assert_eq!(pkg.name, "wget");
    ///     assert_eq!(pkg.formula().name, "wget");
    /// }
    /// ```
    pub fn from(json_str: &str) -> anyhow::Result<Self> {
        let mut pkg: Package = serde_json::from_str(json_str)?;
        // 给 value 赋值
        let value: HashMap<String, Value> = serde_json::from_str(json_str)?;
        pkg.value = value;
        // 给 package 赋值基本属性
        let (name, full_name, tap, desc, homepage) = if pkg.is_cask() {
            let p = pkg.cask();
            let desc = p.desc.clone().unwrap_or_default();
            (
                p.token.clone(),
                p.full_token.clone(),
                p.tap.clone(),
                desc,
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

    /// 判断是否为 `Cask` 包
    /// TODO: 文档自动跳转方法地址
    /// 可以在使用 [`homebrew::info`] 方法获取 `Package` 后调用
    ///
    /// Examples
    ///
    /// ```
    /// extern crate homebrew as brew;
    /// use std::fs;
    ///
    /// use brew::Package;
    ///
    /// fn main() {
    ///     let json_str = fs::read_to_string("examples/data/wget.json").unwrap();
    ///     let pkg = Package::from(&json_str).unwrap();
    ///     // let pkg = brew::info("wget").unwrap();
    ///
    ///     assert!(!pkg.is_cask());
    ///
    ///     let json_str = fs::read_to_string("examples/data/kitty.json").unwrap();
    ///     let pkg = Package::from(&json_str).unwrap();
    ///     // let pkg = brew::info("kitty").unwrap();
    ///
    ///     assert!(pkg.is_cask());
    /// }
    /// ```
    pub fn is_cask(&self) -> bool {
        !self.casks.is_empty()
    }
    pub fn formula(&self) -> &Formula {
        &self.formulae[0]
    }
    pub fn cask(&self) -> &Cask {
        &self.casks[0]
    }

    /// 将包原始 `json` 信息写到 [`std::io::Write`] 中
    ///
    /// Examples
    ///
    /// ```
    /// extern crate homebrew as brew;
    /// use std::{fs::{self, File}, path::Path};
    ///
    /// use brew::Package;
    ///
    /// fn main() {
    ///     let json_str = fs::read_to_string("examples/data/wget.json").unwrap();
    ///     let pkg = Package::from(&json_str).unwrap();
    ///     // let pkg = brew::info("wget").unwrap();
    ///     let save_path = Path::new("target/wget.json");
    ///     let file = File::create(save_path).unwrap();
    ///     pkg.to_writer(&file).unwrap();
    /// }
    /// ```
    pub fn to_writer<T>(&self, writer: T) -> Result<()>
        where T: io::Write
    {
        serde_json::to_writer(writer, &self.value)
    }

    /// 将包原始 `json` 信息写到 [`AsRef<Path>`] 中
    ///
    /// Examples
    ///
    /// ```
    /// extern crate homebrew as brew;
    /// use std::{fs, path::Path};
    ///
    /// use brew::Package;
    ///
    /// fn to_file() {
    ///     let json_str = fs::read_to_string("examples/data/wget.json").unwrap();
    ///     let pkg = Package::from(&json_str).unwrap();
    ///     // let pkg = brew::info("wget").unwrap();
    ///     let save_path = Path::new("target/wget.json");
    ///     pkg.to_file(save_path).unwrap();
    ///
    ///     let pkg_str = serde_json::to_string(&pkg.value).unwrap();
    ///     let save_str = fs::read_to_string(save_path).unwrap();
    ///
    ///     assert_eq!(pkg_str, save_str)
    /// }
    ///
    /// to_file()
    /// ```
    pub fn to_file<T>(&self, path: T) -> anyhow::Result<()>
        where T: AsRef<Path>
    {
        let file =File::create(path)?;
        Ok(self.to_writer(&file)?)
    }

}

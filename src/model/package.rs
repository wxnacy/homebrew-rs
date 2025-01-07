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
    formula: Vec<Formula>,

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
        &self.formula[0]
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


#[cfg(test)]
mod tests {
    use super::Package;

    const JSON_DATA: &str = r#"
{
  "formulae": [
    {
      "name": "gotop",
      "full_name": "gotop",
      "tap": "homebrew/core",
      "oldnames": [

      ],
      "aliases": [

      ],
      "versioned_formulae": [

      ],
      "desc": "Terminal based graphical activity monitor inspired by gtop and vtop",
      "license": "BSD-3-Clause",
      "homepage": "https://github.com/xxxserxxx/gotop",
      "versions": {
        "stable": "4.2.0",
        "head": null,
        "bottle": true
      },
      "urls": {
        "stable": {
          "url": "https://github.com/xxxserxxx/gotop/archive/refs/tags/v4.2.0.tar.gz",
          "tag": null,
          "revision": null,
          "using": null,
          "checksum": "e9d9041903acb6bd3ffe94e0a02e69eea53f1128274da1bfe00fe44331ccceb1"
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
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.arm64_sequoia.bottle.tar.gz",
              "sha256": "5c500e1f45ee743f9628545a7de331e9729a90693be2f5dfea3d297bb7d8772d"
            },
            "arm64_sonoma": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.arm64_sonoma.bottle.tar.gz",
              "sha256": "2ca9e4643126c9f07e728798d9c784b613d03bb6a947b7641b7e6e702f39a94e"
            },
            "arm64_ventura": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.arm64_ventura.bottle.tar.gz",
              "sha256": "59120887e3b8c12144e56945486da0ba9cb53ca1f3c9242e9992c2d79debc119"
            },
            "arm64_monterey": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.arm64_monterey.bottle.tar.gz",
              "sha256": "26f36d53f4f63536d74c2565a66595f2b5658ff0322e123486fdc2df73fcc9ff"
            },
            "arm64_big_sur": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.arm64_big_sur.bottle.tar.gz",
              "sha256": "9a4ec909ce13bf1627374bc35a3e5f55a6e897cb14fb779d677a0c715d580c22"
            },
            "sonoma": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.sonoma.bottle.tar.gz",
              "sha256": "e16d440d5b7db710a63baa9f0d115c89c24a5980d684850df98f1f86f7e7c5ed"
            },
            "ventura": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.ventura.bottle.tar.gz",
              "sha256": "16826d2a09c771408f8686dd3eb2be2e354b457c5e81d14d331498e4a8768e8a"
            },
            "monterey": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.monterey.bottle.tar.gz",
              "sha256": "967cf5ea968270791932cef90aaeb8c131a695e142429d72a1694508c6a01dda"
            },
            "big_sur": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.big_sur.bottle.tar.gz",
              "sha256": "61f87b013e7a20046a34ef65bfeb2cbc68a6e78df6f04baee64fa1bdc5be2d66"
            },
            "catalina": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.catalina.bottle.tar.gz",
              "sha256": "3948c3cf1d4a198462af0bbed422215a12bcd87266af2c9dd629eed8bcc27a6f"
            },
            "x86_64_linux": {
              "cellar": ":any_skip_relocation",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.x86_64_linux.bottle.tar.gz",
              "sha256": "5b4323239fa19e80fcec5d8ef9ba94b5be4015ae9ca0be3c3a74e06a86f15f29"
            }
          }
        }
      },
      "pour_bottle_only_if": null,
      "keg_only": false,
      "keg_only_reason": null,
      "options": [

      ],
      "build_dependencies": [
        "go"
      ],
      "dependencies": [

      ],
      "test_dependencies": [

      ],
      "recommended_dependencies": [

      ],
      "optional_dependencies": [

      ],
      "uses_from_macos": [

      ],
      "uses_from_macos_bounds": [

      ],
      "requirements": [

      ],
      "conflicts_with": [

      ],
      "conflicts_with_reasons": [

      ],
      "link_overwrite": [

      ],
      "caveats": null,
      "installed": [

      ],
      "linked_keg": null,
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
      "tap_git_head": "616640cd0d28150fbab60d51bc2048a1cd576b46",
      "ruby_source_path": "Formula/g/gotop.rb",
      "ruby_source_checksum": {
        "sha256": "4bcce30150beece189d000de2340a508f303dd7e6fd9be2a6a0d88537a2e0deb"
      }
    }
  ],
  "casks": [

  ]
}
"#;

    #[test]
    fn test_from() {
        let pkg: Package = Package::from(JSON_DATA).expect("Failed to parse JSON");
        assert_eq!(pkg.name, "gotop");
    }
}

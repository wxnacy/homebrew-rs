use anyhow::Result;

use crate::{brew, Package};

pub fn info(name: &str) -> Result<Package> {
    let out = brew(format!("info {name} --json=v2").as_str())?;
    let pkg = Package::from(&out)?;
    Ok(pkg)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_info() {
        if let Ok(pkg) = info("rust") {
            let f= pkg.formula();
            assert_eq!(pkg.name, "rust");
            assert_eq!(pkg.full_name, "rust");
            assert_eq!(pkg.tap, "homebrew/care");
            assert_eq!(pkg.desc, "Safe, concurrent, practical language");
            assert_eq!(pkg.homepage, "https://www.rust-lang.org/");

            assert_eq!(f.name, "rust");
            assert_eq!(f.full_name, "rust");
            assert_eq!(f.tap, "homebrew/care");
            assert_eq!(f.oldnames, ["rustfmt"]);
            assert_eq!(f.desc, "Safe, concurrent, practical language");
            assert_eq!(f.homepage, "https://www.rust-lang.org/");
        }
        if let Ok(pkg) = info("kitty") {
            let f= pkg.cask();
            assert_eq!(pkg.name, "kitty");
            assert_eq!(pkg.full_name, "kitty");
            assert_eq!(pkg.tap, "homebrew/cask");
            assert_eq!(pkg.desc, "Safe, concurrent, practical language");
            assert_eq!(pkg.homepage, "https://www.rust-lang.org/");

            assert_eq!(f.token, "kitty");
            assert_eq!(f.full_token, "kitty");
            assert_eq!(f.tap, "homebrew/cask");
            assert_eq!(f.name, ["kitty"]);
            assert_eq!(f.desc, "Safe, concurrent, practical language");
            assert_eq!(f.homepage, "https://www.rust-lang.org/");
            let url = format!("https://github.com/kovidgoyal/kitty/releases/download/v{}/kitty-{}.dmg", &f.version, &f.version);
            assert_eq!(f.url, url);
        }

        let name = "ss";
        if let Err(e) = info(name) {
            eprintln!("{e}");
            assert_eq!(
                e.to_string(),
                format!("Error: No available formula with the name \"{}\".", name)
            );
        }
    }
}


# Homebrew Rust Package

[Homebrew](https://brew.sh/) 在 [Rust](https://www.rust-lang.org/) 中的封装

- [安装](#安装)
- [使用](#使用)
- [info](#info)
  - [--eval-all](#--eval-all)
- [services](#services)
  - [list](#list)
  - [info](#info-1)
  - [stop](#stop)
  - [start](#start)
  - [restart](#restart)
  - [run](#run)
  - [kill](#kill)
  - [cleanup](#cleanup)
- [update](#update)
- [install](#install)
  - [--cask](#--cask)
- [uninstall](#uninstall)
- [upgrade](#upgrade)
- [reinstall](#reinstall)
- [config](#config)

## 安装

```bash
cargo add homebrew
```

## 使用

通过 [examples](https://github.com/wxnacy/homebrew-rs/tree/master/examples) 来了解 homebrew-rs 的使用

### info

查看软件详情，就像运行 `brew info [name] --json=v2`

```rust
extern crate homebrew as brew;

fn main() {
    let pkg = brew::info("wget").unwrap();
    let json = serde_json::to_string_pretty(&pkg).unwrap();
    println!("package name: {}", pkg.name);
    println!("package is_installed: {}", pkg.is_installed());
    println!("{json}");
}
```

<details>
<summary>结果输出</summary>

```bash
package name: wget
package is_installed: true
{
  "casks": [],
  "formulae": [
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
          "branch": null,
          "tag": null,
          "revision": null,
          "using": null,
          "checksum": "766e48423e79359ea31e41db9e5c289675947a7fcf2efdcedb726ac9d0da3784"
        },
        "head": {
          "url": "https://git.savannah.gnu.org/git/wget.git",
          "branch": "master",
          "tag": null,
          "revision": null,
          "using": null,
          "checksum": null
        }
      },
      "revision": 0,
      "version_scheme": 0,
      "bottle": {
        "stable": {
          "rebuild": 0,
          "root_url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles",
          "files": {
            "x86_64_linux": {
              "cellar": "/home/linuxbrew/.linuxbrew/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.x86_64_linux.bottle.tar.gz",
              "sha256": "ab5f3c1c60bef4e2a4781e9b29af8afb48ead837136c419edd7febdf44b59058"
            },
            "arm64_sonoma": {
              "cellar": "/opt/homebrew/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.arm64_sonoma.bottle.tar.gz",
              "sha256": "4d180cd4ead91a34e2c2672189fc366b87ae86e6caa3acbf4845b272f57c859a"
            },
            "ventura": {
              "cellar": "/usr/local/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.ventura.bottle.tar.gz",
              "sha256": "78cee523a9b58a7b824b51767935f68c9838e9f673e70d001982858001e766ff"
            },
            "arm64_sequoia": {
              "cellar": "/opt/homebrew/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.arm64_sequoia.bottle.tar.gz",
              "sha256": "a93dd95c5d63036e026b526e000d33fae7fb44d9a8fda5afc89bff112438c6b3"
            },
            "sonoma": {
              "cellar": "/usr/local/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.sonoma.bottle.tar.gz",
              "sha256": "5650778a8e7a60c2dea9412dd21d2f5e8ff4f224dbefbdf54924b99012062edc"
            },
            "arm64_ventura": {
              "cellar": "/opt/homebrew/Cellar",
              "url": "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/wget-1.25.0.arm64_ventura.bottle.tar.gz",
              "sha256": "7fce09705a52a2aff61c4bdd81b9d2a1a110539718ded2ad45562254ef0f5c22"
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
        {
          "since": null
        }
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
      "tap_git_head": "bcbeee8ae56945a92cdebec76da6bfa1cd96d38f",
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
          {
            "since": null
          }
        ]
      }
    }
  ]
}
```

</details>

#### --eval-all

获取全部包(包括 `formula` 和 `cask`，下载和没下载的)

就像执行 `brew info --eval-all --json=v2` 命令

```rust
extern crate homebrew as brew;

fn main() {
    let pkg = brew::info_all().unwrap();
    let mut formula_count = 0;
    for p in pkg.formulae().iter() {
        if p.is_installed() {
            formula_count += 1;
        }
    }
    println!("Formula package total: {} installed:\t {formula_count}", pkg.formulae().len());

    let mut cask_count = 0;
    for cask in pkg.casks().iter() {
        if cask.is_installed() {
            cask_count += 1;
        }
    }
    println!("Cask package total: {} installed:\t {cask_count}", pkg.casks().len());
}
```

```bash
Formula package total: 7393 installed:   251
Cask package total: 7289 installed:      13
```

### services

#### list

列出所有服务，就像运行 `brew services`

```rust
use homebrew;

let out = homebrew::services().unwrap();
println!("{out:#?}")
```

<details>
<summary>结果输出</summary>

```rust
[
    Service {
        name: "mongodb-community@7.0",
        status: None,
        user: Some(
            "wxnacy",
        ),
        file: "/Users/wxnacy/Library/LaunchAgents/homebrew.mxcl.mongodb-community@7.0.plist",
        exit_code: None,
    },
    Service {
        name: "mpd",
        status: Started,
        user: Some(
            "wxnacy",
        ),
        file: "/Users/wxnacy/Library/LaunchAgents/homebrew.mxcl.mpd.plist",
        exit_code: Some(
            0,
        ),
    },
    Service {
        name: "postgresql@14",
        status: Started,
        user: Some(
            "wxnacy",
        ),
        file: "/Users/wxnacy/Library/LaunchAgents/homebrew.mxcl.postgresql@14.plist",
        exit_code: Some(
            0,
        ),
    },
    Service {
        name: "redis",
        status: Started,
        user: Some(
            "wxnacy",
        ),
        file: "/Users/wxnacy/Library/LaunchAgents/homebrew.mxcl.redis.plist",
        exit_code: Some(
            0,
        ),
    },
    Service {
        name: "unbound",
        status: None,
        user: None,
        file: "/opt/homebrew/opt/unbound/homebrew.mxcl.unbound.plist",
        exit_code: None,
    },
]
```

</details>

#### info

查询服务详情，就像运行 `brew services info [name] --json`

```rust
use homebrew;

let out = homebrew::services_info("unbound").unwrap();
println!("{out:#?}")
```

```rust
ServiceInfo {
    name: "unbound",
    service_name: "homebrew.mxcl.unbound",
    running: false,
    loaded: false,
    schedulable: false,
    pid: None,
    exit_code: None,
    user: None,
    status: None,
    file: "/opt/homebrew/opt/unbound/homebrew.mxcl.unbound.plist",
    command: "/opt/homebrew/opt/unbound/sbin/unbound -d -c /opt/homebrew/etc/unbound/unbound.conf",
    working_dir: None,
    root_dir: None,
    log_path: None,
    error_log_path: None,
    interval: None,
    cron: None,
}
```

#### stop

停止服务并注销自启动，就像运行 `brew services stop [name]`

__确保你的机器上已经安装 mpd，或者其他可以启动的服务__

```rust
use homebrew;

let out = homebrew::services_stop("mpd").unwrap();
println!("{out}")
```

```bash
Stopping `mpd`... (might take a while)
==> Successfully stopped `mpd` (label: homebrew.mxcl.mpd)
```

#### start

启动服务并注册自启动，就像运行 `brew services start [name]`

__确保你的机器上已经安装 mpd，或者其他可以启动的服务__

```rust
use homebrew;

let out = homebrew::services_start("mpd").unwrap();
println!("{out}")
```

```bash
==> Successfully started `mpd` (label: homebrew.mxcl.mpd)
```

#### restart

重启服务并注册自启动，就像运行 `brew services restart [name]`

__确保你的机器上已经安装 mpd，或者其他可以启动的服务__

```rust
use homebrew;

let out = homebrew::services_restart("mpd").unwrap();
println!("{out}")
```

```bash
Stopping `mpd`... (might take a while)
==> Successfully stopped `mpd` (label: homebrew.mxcl.mpd)
==> Successfully started `mpd` (label: homebrew.mxcl.mpd)
```

#### run

启动服务但不注册自启动，就像运行 `brew services run [name]`

__确保你的机器上已经安装 mpd，或者其他可以启动的服务__

```rust
use homebrew;

let out = homebrew::services_run("mpd").unwrap();
println!("{out}")
```

```bash
==> Successfully ran `mpd` (label: homebrew.mxcl.mpd)
```

#### kill

终止服务但保持自启动，就像运行 `brew services kill [name]`

__确保你的机器上已经安装 mpd，或者其他可以启动的服务__

```rust
use homebrew;

let out = homebrew::services_kill("mpd").unwrap();
println!("{out}")
```

#### cleanup

移除所有没用的服务，就像运行 `brew services cleanup`

__确保你的机器上已经安装 mpd，或者其他可以启动的服务__

```rust
use homebrew;

let out = homebrew::services_cleanup().unwrap();
println!("{out}")
```

### update

执行完在输出结果

```rust
use homebrew;

let out = homebrew::update().unwrap();
println!("{out}")
```

实时输出结果

```rust
use homebrew;

homebrew::update_spawn().unwrap();
```

```bash
HOMEBREW_BREW_GIT_REMOTE set: using https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git as the Homebrew/brew Git remote.
==> Updating Homebrew...
Already up-to-date.
```

### install

执行完在输出结果

```rust
use homebrew;

let out = homebrew::install("gotop").unwrap();
println!("{out}")
```

实时输出结果

```rust
use homebrew;

homebrew::install_spawn("gotop").unwrap();
```

```bash
==> Fetching gotop
==> Downloading https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gotop-4.2.0.arm64_sequoia.bottle.tar.gz
Already downloaded: /Users/wxnacy/Library/Caches/Homebrew/downloads/44c51f808e5d60cecddb47b1a8d92b36027b60dfe85314dade7b9de690d8e8ce--gotop-4.2.0.arm64_sequoia.bottle.tar.gz
==> Pouring gotop-4.2.0.arm64_sequoia.bottle.tar.gz
🍺  /opt/homebrew/Cellar/gotop/4.2.0: 7 files, 10.7MB
==> Running `brew cleanup gotop`...
Disable this behaviour by setting HOMEBREW_NO_INSTALL_CLEANUP.
Hide these hints with HOMEBREW_NO_ENV_HINTS (see `man brew`).
```

#### --cask

执行完在输出结果

```rust
use homebrew;

let out = homebrew::install_cask("kitty").unwrap();
println!("{out}")
```

实时输出结果

```rust
use homebrew;

homebrew::install_cask_spawn("kitty").unwrap();
```

```bash
==> Downloading https://github.com/keycastr/keycastr/releases/download/v0.10.2/KeyCastr.app.zip
Already downloaded: /Users/wxnacy/Library/Caches/Homebrew/downloads/5922fdb78b047a1dc34313507721add8fd17e61928cdcb92c0039b599098ca38--KeyCastr.app.zip
==> Installing Cask keycastr
==> Moving App 'KeyCastr.app' to '/Applications/KeyCastr.app'
```

### uninstall

执行完在输出结果

```rust
use homebrew;

let out = homebrew::uninstall("gotop").unwrap();
println!("{out}")
```

```bash
Uninstalling /opt/homebrew/Cellar/gotop/4.2.0... (7 files, 10.7MB)
```

### upgrade

执行完在输出结果

```rust
use homebrew;

let out = homebrew::upgrade("superfile").unwrap();
println!("{out}")
```

实时输出结果

```rust
use homebrew;

homebrew::upgrade_spawn("superfile").unwrap();
```

```bash
==> Upgrading 1 outdated package:
superfile 1.1.6 -> 1.1.7.1
==> Fetching superfile
==> Downloading https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/superfile-1.1.7.1.arm64_sequoia.bottle.tar.gz
############################################################################################################ 100.0%
==> Upgrading superfile
  1.1.6 -> 1.1.7.1
==> Pouring superfile-1.1.7.1.arm64_sequoia.bottle.tar.gz
🍺  /opt/homebrew/Cellar/superfile/1.1.7.1: 7 files, 19.3MB
==> Running `brew cleanup superfile`...
Disable this behaviour by setting HOMEBREW_NO_INSTALL_CLEANUP.
Hide these hints with HOMEBREW_NO_ENV_HINTS (see `man brew`).
Removing: /opt/homebrew/Cellar/superfile/1.1.6... (7 files, 19.3MB)
Removing: /Users/wxnacy/Library/Caches/Homebrew/superfile--1.1.6.arm64_sequoia.bottle.tar.gz... (6.3MB)
```

### reinstall

执行完在输出结果

```rust
use homebrew;

let out = homebrew::reinstall("gtop").unwrap();
println!("{out}")
```

实时输出结果

```rust
use homebrew;

homebrew::reinstall_spawn("gtop").unwrap();
```

```bash
==> Fetching gtop
==> Downloading https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/gtop-1.1.5.arm64_sequoia.bottle.1.tar.gz
Already downloaded: /Users/wxnacy/Library/Caches/Homebrew/downloads/eaf8475135f08001cc1e02022d76c5bc9fb1ab26d4be2a89e9bf1d738eb86b8e--gtop-1.1.5.arm64_sequoia.bottle.1.tar.gz
==> Reinstalling gtop
==> Pouring gtop-1.1.5.arm64_sequoia.bottle.1.tar.gz
🍺  /opt/homebrew/Cellar/gtop/1.1.5: 1,872 files, 11.7MB
==> Running `brew cleanup gtop`...
Disable this behaviour by setting HOMEBREW_NO_INSTALL_CLEANUP.
Hide these hints with HOMEBREW_NO_ENV_HINTS (see `man brew`).
```

### config

执行完在输出结果

```rust
use homebrew;

let cfg = homebrew::config().unwrap();
println!("{cfg:#?}")
```

```bash
Config {
    version: "4.4.15-56-g6aac197",
    origin: "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git",
    head: "6aac197d556f60e82490dcb46fcbe7090c8934e9",
    last_commit: "2 days ago",
    branch: "master",
    core_tap_json: "10 Jan 14:49 UTC",
    core_cask_tap_json: "10 Jan 14:49 UTC",
    prefix: "/opt/homebrew",
    api_domain: "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/api",
    bottle_domain: "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles",
    brew_git_remote: "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git",
    cask_opts: [],
    core_git_remote: "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/homebrew-core.git",
    editor: "nvim",
    make_jobs: 12,
    pip_index_url: "https://pypi.tuna.tsinghua.edu.cn/simple",
    sorbet_runtime: "set",
    ruby: "3.3.6 => /opt/homebrew/Library/Homebrew/vendor/portable-ruby/3.3.6/bin/ruby",
    cpu: "dodeca-core 64-bit arm_brava",
    clang: "16.0.0 build 1600",
    git: "2.47.1 => /opt/homebrew/bin/git",
    curl: "8.7.1 => /usr/bin/curl",
    macos: "15.1-arm64",
    clt: "16.2.0.0.1.1733547573",
    xcode: "N/A",
    rosetta2: false,
}
```

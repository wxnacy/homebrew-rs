# Homebrew Rust Package

[Homebrew](https://brew.sh/) 在 [Rust](https://www.rust-lang.org/) 中的封装

## 使用

通过 [examples](https://github.com/wxnacy/homebrew-rs/tree/master/examples) 来了解 homebrew-rs 的使用

```rust
extern crate homebrew as brew;

/// cargo run --example info
///
/// {
///   "casks": [],
///   "formulae": [
///     {
///       "name": "wget",
///       "full_name": "wget",
///       "tap": "homebrew/core",
///       "oldnames": [],
///       "aliases": [],
///       "versioned_formulae": [],
///       "desc": "Internet file retriever",
///       "license": "GPL-3.0-or-later",
///       "homepage": "https://www.gnu.org/software/wget/",
///
///       ...
///     }
///   ]
/// }
/// package name: wget
/// package is_installed: true
fn main() {
    let pkg = brew::info("wget").unwrap();
    let json = serde_json::to_string_pretty(&pkg).unwrap();
    println!("{json}");
    println!("package name: {}", pkg.name);
    println!("package is_installed: {}", pkg.is_installed());
}
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

*确保你的机器上已经安装 mpd，或者其他可以启动的服务*

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

*确保你的机器上已经安装 mpd，或者其他可以启动的服务*

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

*确保你的机器上已经安装 mpd，或者其他可以启动的服务*

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

*确保你的机器上已经安装 mpd，或者其他可以启动的服务*

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

*确保你的机器上已经安装 mpd，或者其他可以启动的服务*

```rust
use homebrew;

let out = homebrew::services_kill("mpd").unwrap();
println!("{out}")
```

#### cleanup

移除所有没用的服务，就像运行 `brew services cleanup`

*确保你的机器上已经安装 mpd，或者其他可以启动的服务*

```rust
use homebrew;

let out = homebrew::services_cleanup().unwrap();
println!("{out}")
```

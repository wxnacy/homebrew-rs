use crate::{brew, Service, ServiceInfo};


/// 列出所有服务，就像运行 `brew services`
///
/// Examples
///
/// ```
/// use homebrew;
/// use homebrew::ServiceStatus;
///
/// let srvs = homebrew::services().unwrap();
///
/// let srv = srvs.last().unwrap();
///
/// assert_eq!(srv.name, "unbound");
/// assert_eq!(srv.status, ServiceStatus::None);
/// assert_eq!(srv.user, None);
/// assert_eq!(srv.file, "/opt/homebrew/opt/unbound/homebrew.mxcl.unbound.plist");
/// assert_eq!(srv.exit_code, None);
/// ```
pub fn services() -> anyhow::Result<Vec<Service>>{
    let out = brew("services --json")?;
    let srvs: Vec<Service> = serde_json::from_str(&out)?;
    Ok(srvs)
}

/// 查询服务详情，就像运行 `brew services info [name] --json`
///
/// Examples
///
/// ```
/// use homebrew;
/// use homebrew::ServiceStatus;
///
/// let info = homebrew::services_info("unbound").unwrap();
///
/// assert_eq!(info.name, "unbound");
/// assert_eq!(info.status, ServiceStatus::None);
/// assert_eq!(info.user, None);
/// assert_eq!(info.pid, None);
/// assert_eq!(info.file, "/opt/homebrew/opt/unbound/homebrew.mxcl.unbound.plist");
/// assert_eq!(info.exit_code, None);
///
/// let info = homebrew::services_info("ss").unwrap_err();
///
/// assert_eq!(info.to_string(), "Error: No available formula with the name \"ss\".")
/// ```
pub fn services_info(name: &str) -> anyhow::Result<ServiceInfo>{
    let out = brew(format!("services info {name} --json").as_str())?;
    let infos: Vec<ServiceInfo> = serde_json::from_str(&out)?;
    Ok(infos[0].clone())
}

/// 启动服务并注册自启动，就像运行 `brew services start [name]`
///
/// Examples
///
/// ```
/// use homebrew;
///
/// // 确保你的机器上已经安装 mpd
/// let name = "mpd";
/// homebrew::services_stop(&name).unwrap();
/// let info = homebrew::services_start(&name).unwrap();
/// assert_eq!(info, "==> Successfully started `mpd` (label: homebrew.mxcl.mpd)");
///
/// let info = homebrew::services_start(&name).unwrap();
/// assert_eq!(info, "Service `mpd` already started, use `brew services restart mpd` to restart.");
///
/// let info = homebrew::services_start("ss").unwrap_err();
///
/// assert_eq!(info.to_string(), "Error: No available formula with the name \"ss\".")
/// ```
pub fn services_start(name: &str) -> anyhow::Result<String>{
    brew(format!("services start {name}").as_str())
}

/// 启动服务但不注册自启动，就像运行 `brew services run [name]`
///
/// Examples
///
/// ```
/// use homebrew;
///
/// // 确保你的机器上已经安装 mpd
/// let name = "mpd";
/// homebrew::services_stop(&name).unwrap();
/// let info = homebrew::services_run(&name).unwrap();
/// assert_eq!(info, "==> Successfully ran `mpd` (label: homebrew.mxcl.mpd)");
///
/// let info = homebrew::services_run(&name).unwrap();
/// assert_eq!(info, "Service `mpd` already running, use `brew services restart mpd` to restart.");
///
/// let info = homebrew::services_run("ss").unwrap_err();
///
/// assert_eq!(info.to_string(), "Error: No available formula with the name \"ss\".")
/// ```
pub fn services_run(name: &str) -> anyhow::Result<String>{
    brew(format!("services run {name}").as_str())
}

/// 停止服务并注销自启动，就像运行 `brew services stop [name]`
///
/// Examples
///
/// ```
/// use homebrew;
/// use homebrew::ServiceStatus;
///
/// // 确保你的机器上已经安装 mpd
/// homebrew::services_start("mpd").unwrap();
/// let info = homebrew::services_stop("mpd").unwrap();
/// assert_eq!(info, "Stopping `mpd`... (might take a while)\n==> Successfully stopped `mpd` (label: homebrew.mxcl.mpd)");
///
/// let info = homebrew::services_stop("mpd").unwrap_err();
/// assert_eq!(info.to_string(), "Warning: Service `mpd` is not started.");
///
/// let info = homebrew::services_stop("ss").unwrap_err();
///
/// assert_eq!(info.to_string(), "Error: No available formula with the name \"ss\".")
/// ```
pub fn services_stop(name: &str) -> anyhow::Result<String>{
    brew(format!("services stop {name}").as_str())
}

/// 重启服务并注册自启动，就像运行 `brew services restart [name]`
///
/// Examples
///
/// ```
/// use homebrew;
/// use homebrew::ServiceStatus;
///
/// // 确保你的机器上已经安装 mpd
/// homebrew::services_start("mpd").unwrap();
/// let info = homebrew::services_restart("mpd").unwrap();
/// assert_eq!(info, r#"Stopping `mpd`... (might take a while)
/// ==> Successfully stopped `mpd` (label: homebrew.mxcl.mpd)
/// ==> Successfully started `mpd` (label: homebrew.mxcl.mpd)"#);
///
/// homebrew::services_stop("mpd").unwrap();
/// let info = homebrew::services_restart("mpd").unwrap();
/// assert_eq!(info, "==> Successfully started `mpd` (label: homebrew.mxcl.mpd)");
///
/// ```
pub fn services_restart(name: &str) -> anyhow::Result<String>{
    brew(format!("services restart {name}").as_str())
}

/// 终止服务但保持自启动，就像运行 `brew services kill [name]`
pub fn services_kill(name: &str) -> anyhow::Result<String>{
    brew(format!("services kill {name}").as_str())
}

/// 移除所有没用的服务，就像运行 `brew services cleanup`
pub fn services_cleanup() -> anyhow::Result<String>{
    brew(format!("services cleanup").as_str())
}

use serde::{Serialize, Deserialize};
use std::str::FromStr;
use std::fmt;

// 定义状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")] // 使得枚举可以接受小写字符串
pub enum ServiceStatus {
    None,
    Started,
    Error,
}

// 实现从字符串转换为枚举
impl FromStr for ServiceStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "none" => Ok(ServiceStatus::None),
            "started" => Ok(ServiceStatus::Started),
            "error" => Ok(ServiceStatus::Error),
            _ => Err(()),
        }
    }
}

// 实现 Display trait 以便打印
impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceStatus::None => write!(f, "none"),
            ServiceStatus::Started => write!(f, "started"),
            ServiceStatus::Error => write!(f, "error"),
        }
    }
}

/// `Service` 的结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub status: ServiceStatus,
    pub user: Option<String>,
    pub file: String,
    pub exit_code: Option<i8>,
}

impl Service {
    /// 通过 `json` 字符串来新建结构体
    ///
    /// Examples
    ///
    /// ```
    /// use homebrew;
    ///
    /// let json_str = r#"
    /// {
    ///     "name": "mongodb-community@7.0",
    ///     "status": "none",
    ///     "user": "wxnacy",
    ///     "file": "/Users/wxnacy/Library/LaunchAgents/homebrew.mxcl.mongodb-community@7.0.plist",
    ///     "exit_code": null
    /// }
    /// "#;
    ///
    /// let s = homebrew::Service::from(&json_str).unwrap();
    ///
    /// assert_eq!(s.name, "mongodb-community@7.0");
    /// ```
    pub fn from(json_str: &str) -> anyhow::Result<Self> {
        let pkg: Self = serde_json::from_str(json_str)?;
        Ok(pkg)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub service_name: String,
    pub running: bool,
    pub loaded: bool,
    pub schedulable: bool,
    pub pid: Option<u32>,
    pub exit_code: Option<i32>,
    pub user: Option<String>,
    pub status: ServiceStatus, // 使用枚举代替字符串
    pub file: String,
    pub command: String,
    pub working_dir: Option<String>,
    pub root_dir: Option<String>,
    pub log_path: Option<String>,
    pub error_log_path: Option<String>,
    pub interval: Option<String>,
    pub cron: Option<String>,
}

impl ServiceInfo {
    /// 通过 `json` 字符串来新建结构体
    ///
    /// Examples
    ///
    /// ```
    /// use homebrew;
    /// use homebrew::ServiceStatus;
    ///
    /// let json_str = r#"
    /// {
    ///     "name": "redis",
    ///     "service_name": "homebrew.mxcl.redis",
    ///     "running": false,
    ///     "loaded": false,
    ///     "schedulable": false,
    ///     "pid": null,
    ///     "exit_code": null,
    ///     "user": null,
    ///     "status": "none",
    ///     "file": "/opt/homebrew/opt/redis/homebrew.mxcl.redis.plist",
    ///     "command": "/opt/homebrew/opt/redis/bin/redis-server /opt/homebrew/etc/redis.conf",
    ///     "working_dir": "/opt/homebrew/var",
    ///     "root_dir": null,
    ///     "log_path": "/opt/homebrew/var/log/redis.log",
    ///     "error_log_path": "/opt/homebrew/var/log/redis.log",
    ///     "interval": null,
    ///     "cron": null
    /// }
    /// "#;
    ///
    /// let s = homebrew::ServiceInfo::from(&json_str).unwrap();
    ///
    /// assert_eq!(s.name, "redis");
    /// assert_eq!(s.status, ServiceStatus::None);
    /// ```
    pub fn from(json_str: &str) -> anyhow::Result<Self> {
        let pkg: Self = serde_json::from_str(json_str)?;
        Ok(pkg)
    }
}

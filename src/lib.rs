//! 暂时只实现了 `brew` 基础命令，更多命令可以通过 [`Brew`] 构造实现
//!
//! 命令执行前均默认设置环境变量 `HOMEBREW_NO_AUTO_UPDATE=1` 但 `update()` 除外
mod model;
mod config;
mod command;

pub use model::formula::Formula;
pub use model::cask::Cask;
pub use model::config::{Config, Env};
pub use model::package::Package;
pub use model::service::{Service, ServiceInfo, ServiceStatus};
pub use command::info::{info, info_all};
pub use command::list::{list, list_formulae, list_cask};
pub use command::cmd::{brew, Brew, brew_spawn};
pub use command::config::{
    caskroom,
    cache,
    cellar,
    config,
    env,
    env_shell,
    version,
    repository,
    prefix,
};
pub use command::install::{
    update,
    update_spawn,
    install,
    install_spawn,
    install_cask,
    install_cask_spawn,
    uninstall,
    upgrade,
    upgrade_spawn,
    reinstall,
    reinstall_spawn,
};
pub use command::search::search;
pub use command::service::{
    services,
    services_run,
    services_info,
    services_stop,
    services_start,
    services_restart,
    services_kill,
    services_cleanup,
};

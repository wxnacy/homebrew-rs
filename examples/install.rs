
extern crate homebrew as brew;

fn main() {
    // 更新
    // let out = brew::update().unwrap();
    brew::update_spawn().unwrap();

    // 下载
    // let out = brew::install("gotop").unwrap();
    brew::install_spawn("gotop").unwrap();

    // 升级
    // let out = brew::upgrade("gotop").unwrap();
    brew::upgrade_spawn("gotop").unwrap();

    // 重新下载
    // let out = brew::reinstall("gtop").unwrap();
    brew::reinstall_spawn("gtop").unwrap();

    // 卸载
    let out = brew::uninstall("gotop").unwrap();
    println!("{out}");
}

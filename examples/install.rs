
extern crate homebrew as brew;

fn main() {
    // let out = brew::update().unwrap();
    brew::update_spawn().unwrap();
    // let out = brew::install("gotop").unwrap();
    brew::install_spawn("gotop").unwrap();
    // let out = brew::upgrade("gotop").unwrap();
    brew::upgrade_spawn("gotop").unwrap();
    let out = brew::uninstall("gotop").unwrap();
    println!("{out}");
}

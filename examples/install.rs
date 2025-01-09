
extern crate homebrew as brew;

fn main() {
    brew::update_spawn().unwrap();
    brew::install_spawn("gotop").unwrap();
    let out = brew::uninstall("gotop").unwrap();
    println!("{out}");
}

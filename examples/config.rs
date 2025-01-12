
fn main() {
    let cfg = homebrew::config().unwrap();
    println!("{cfg:#?}");

    let cfg = homebrew::env().unwrap();
    println!("{cfg:#?}");

    let out = homebrew::env_shell().unwrap();
    println!("{out}");

    let out = homebrew::prefix().unwrap();
    println!("{out}");

    let out = homebrew::caskroom().unwrap();
    println!("{out}");

    let out = homebrew::cellar().unwrap();
    println!("{out}");

    let out = homebrew::cache().unwrap();
    println!("{out}");

    let out = homebrew::repository().unwrap();
    println!("{out}");

}

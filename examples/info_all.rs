extern crate homebrew as brew;

/// cargo run --example info_all
///
/// Formula package total: 7393 installed:   251
/// Cask package total: 7289 installed:      13
fn main() {
    let pkg = brew::info_all().unwrap();
    // println!("Formula package total:\t {}", pkg.formulae().len());
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

use brew::Brew;

extern crate homebrew as brew;

fn main() {
    let out = brew::brew("--prefix").unwrap();

    let out1 = Brew::default()
        .set_cmd("--prefix")
        .output()
        .unwrap();

    let mut b2 = Brew::new("--prefix");
    b2.set_env_no_auto_update();
    let out2 = b2.output().unwrap();


    assert_eq!(out, out1);
    assert_eq!(out2, out1);
}

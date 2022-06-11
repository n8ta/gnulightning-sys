use std::process::Command;
use std::env;

fn main() {

    env::set_current_dir("include/lightning-2.1.3").expect("unable to find include/lighting2.1.3 directory");
    Command::new("./configure")
        .spawn().unwrap();

    Command::new("make")
        .spawn().unwrap();
    Command::new("make").args(&["install"])
        .spawn().unwrap();

    let cwd = env::current_dir().unwrap()
        .join("lib")
        .join(".libs");
    let path = cwd.to_str().unwrap();
    println!("cargo:rustc-link-search={}", path);
    println!("cargo:rustc-link-lib={}","lightning.1");
    println!("cargo:rerun-if-changed=build.rs");
}

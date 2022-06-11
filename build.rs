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
}
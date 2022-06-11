use std::process::Command;
use std::env;
use std::path::Path;

fn main() {

    Command::new("./include/lightning-2.1.3/configure")
        .spawn().unwrap();

    env::set_current_dir("include/lightning-2.1.3").expect("unable to find include/lighting2.1.3 directory");
    Command::new("make")
        .spawn().unwrap();
}
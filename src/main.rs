extern crate positioned_io;
use positioned_io::Size;

use std::fs::File;
use std::str;

fn main() {
    let file = File::open("Cargo.toml").unwrap();
    println!("{}", file.size().unwrap().unwrap());
}

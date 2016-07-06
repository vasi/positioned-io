extern crate positioned_io;
use positioned_io::ReadAt;

use std::fs::File;
use std::str;

fn main() {
    let file = File::open("Cargo.toml").unwrap();
    let mut buf = [0; 4];
    file.read_exact_at(buf.as_mut(), 10).unwrap();
    println!("{}", str::from_utf8(buf.as_ref()).unwrap());
}

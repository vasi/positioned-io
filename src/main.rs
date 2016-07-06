extern crate positioned_io;
use positioned_io::SizeCursor;

use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use std::str;

fn main() {
    let file = File::open("Cargo.toml").unwrap();
    let mut curs = SizeCursor::new_pos(file, 10);
    let mut buf = [0; 4];
    curs.seek(SeekFrom::End(-4)).unwrap();
    curs.read(&mut buf).unwrap();
    println!("{}", str::from_utf8(buf.as_ref()).unwrap());
}

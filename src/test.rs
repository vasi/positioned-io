use std::fs::File;
use std::str;

use ::{ReadAt, Size};

fn testReadAt() {
    let file = File::open("Cargo.toml").unwrap();
    let mut buf = [0; 4];
    file.read_exact_at(buf.as_mut(), 10).unwrap();
    let read = str::from_utf8(buf.as_ref()).unwrap();
    assert_eq!(read, "name");
}

fn testSize() {
    let file = File::open("Cargo.toml").unwrap();
    let size = file.size().unwrap().unwrap();
    assert!(size > 0);
}

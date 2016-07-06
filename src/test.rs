use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::File;
use std::str;

use ::{ReadAt, Size, Cursor};

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

fn testCursor() {
    let file = File::open("Cargo.toml").unwrap();
    let mut curs = Cursor::new_pos(file, 10);
    let mut buf = [0; 4];
    curs.read(&mut buf).unwrap();
    assert_eq!("name", str::from_utf8(buf.as_ref()).unwrap());
    curs.seek(SeekFrom::Current(4)).unwrap();
    curs.read(&mut buf).unwrap();
    assert_eq!("posi", str::from_utf8(buf.as_ref()).unwrap());
}

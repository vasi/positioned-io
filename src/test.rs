use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use std::str;

extern crate byteorder;
use self::byteorder::LittleEndian;

use super::{ReadAt, Size, Cursor, SizeCursor, ByteIo};

#[test]
fn test_read_at() {
    let file = File::open("Cargo.toml").unwrap();
    let mut buf = [0; 4];
    file.read_exact_at(10, buf.as_mut()).unwrap();
    let s = str::from_utf8(buf.as_ref()).unwrap();
    assert_eq!(s, "name");
}

#[test]
fn test_size() {
    let file = File::open("Cargo.toml").unwrap();
    let size = file.size().unwrap().unwrap();
    assert!(size > 0);
}

#[test]
fn test_cursor() {
    let file = File::open("Cargo.toml").unwrap();
    let mut curs = Cursor::new_pos(file, 10);
    let mut buf = [0; 4];
    {
        curs.read_exact(&mut buf).unwrap();
        let s = str::from_utf8(buf.as_ref()).unwrap();
        assert_eq!("name", s);
    }
    curs.seek(SeekFrom::Current(4)).unwrap();
    curs.read_exact(&mut buf).unwrap();
    let s = str::from_utf8(buf.as_ref()).unwrap();
    assert_eq!("posi", s);
}

#[test]
fn test_size_cursor() {
    let file = File::open("Cargo.toml").unwrap();
    let mut curs = SizeCursor::new_pos(file, 10);
    let mut buf = [0; 4];
    curs.seek(SeekFrom::End(-2)).unwrap();
    assert_eq!(2, curs.read(&mut buf).unwrap());
    let s = str::from_utf8(buf.as_ref()).unwrap();
    assert!(s.contains("\n"));
}

#[test]
fn test_byteio() {
    let file = File::open("Cargo.toml").unwrap();
    let io: ByteIo<_, LittleEndian> = ByteIo::new(file);
    let r = io.read_i32_at(3).unwrap();
    assert_eq!(0x67616b63, r);
}

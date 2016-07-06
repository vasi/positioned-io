mod byteorder;

extern crate byteorder as extbyteorder;

use std::io::{self, Error, ErrorKind, Result, Read, Write};

// Read at a position.
pub trait ReadAt {
    fn read_at(&self, buf: &mut [u8], pos: u64) -> Result<usize>;
    fn read_exact_at(&self, mut buf: &mut [u8], mut pos: u64) -> Result<()> {
        while !buf.is_empty() {
            match self.read_at(buf, pos) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                    pos += n as u64;
                },
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {},
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            Err(Error::new(ErrorKind::UnexpectedEof, "failed to fill whole buffer"))
        } else {
            Ok(())
        }
    }
}

// Write at a position.
pub trait WriteAt {
    fn write_at(&mut self, buf: &[u8], pos: u64) -> Result<usize>;
    fn write_all_at(&mut self, mut buf: &[u8], mut pos: u64) -> Result<()> {
        while !buf.is_empty() {
            match self.write_at(buf, pos) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &tmp[n..];
                    pos += n as u64;
                },
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {},
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    fn flush(&mut self) -> Result<()>;
}

// Turn a positioned writer into a cursor.
pub struct Cursor<I>(pub io::Cursor<I>);
impl<I> Cursor<I> {
    pub fn new(io: I, pos: u64) -> Self {
        let mut curs = io::Cursor::new(io);
        curs.set_position(pos);
        Cursor(curs)
    }
}
impl<T> Read for Cursor<T> where T: ReadAt {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }
}
impl<T> Write for Cursor<T> where T: WriteAt {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        unimplemented!()
    }
    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}

// Implementation for Unix files.
#[cfg(unix)]
mod unix;

// Other implementations?
// - Windows files
// - Byte arrays


#[cfg(test)]
mod test;

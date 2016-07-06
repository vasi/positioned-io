mod byteorder;

extern crate byteorder as extbyteorder;
use extbyteorder::ByteOrder;

use std::fs::File;
use std::io::{self, Result, Read, Write};

// Read at a position.
pub trait ReadAt {
    fn read_at(&self, buf: &mut [u8], pos: u64) -> Result<usize>;
    fn read_exact_at(&self, buf: &mut [u8], pos: u64) -> Result<()> {
        unimplemented!()
    }
}

// Write at a position.
pub trait WriteAt {
    fn write_at(&mut self, buf: &mut [u8], pos: u64) -> Result<usize>;
    fn write_all_at(&mut self, buf: &mut [u8], pos: u64) -> Result<()> {
        unimplemented!()
    }
    fn flush(&mut self) -> Result<()>;
}

// Turn a positioned writer into a cursor.
struct Cursor<I>(io::Cursor<I>);
impl<I> Cursor<I> {
    fn new(io: I, pos: u64) -> Self {
        unimplemented!()
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
impl ReadAt for File {
    fn read_at(&self, buf: &mut [u8], pos: u64) -> Result<usize> {
        unimplemented!()
    }
}
#[cfg(unix)]
impl WriteAt for File {
    fn write_at(&mut self, buf: &mut [u8], pos: u64) -> Result<usize> {
        unimplemented!()
    }
    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}

// Other implementations?
// - Windows files
// - Byte arrays


#[cfg(test)]
mod tests {
}

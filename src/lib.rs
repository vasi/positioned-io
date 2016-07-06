mod byteorder;
pub use byteorder::{ReadBytesExt, WriteBytesExt};
pub use byteorder::byteio::ByteIo;
mod cursor;
pub use cursor::{Cursor, SizeCursor};


extern crate byteorder as extbyteorder;

use std::io::{Error, ErrorKind, Result};

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
                }
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
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
                }
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    fn flush(&mut self) -> Result<()>;
}

// Trait to get the size of an IO object.
pub trait Size {
    // Can legitimiately return None if there's no size, eg: a socket.
    fn size(&self) -> Result<Option<u64>>;
}

// Implementation for Unix files.
#[cfg(unix)]
mod unix;

// Other implementations?
// - Windows files
// - Byte arrays

#[cfg(test)]
mod test;

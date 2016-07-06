use std::io::{Result, Read, Write, Seek, SeekFrom, Error, ErrorKind};
use ::{ReadAt, WriteAt, Size};

// Turn a positioned writer into a cursor.
pub struct Cursor<I> {
    io: I,
    pos: u64,
}
impl<I> Cursor<I> {
    pub fn new_pos(io: I, pos: u64) -> Self {
        Cursor { io: io, pos: pos }
    }
    pub fn new(io: I) -> Self {
        Self::new_pos(io, 0)
    }

    pub fn into_inner(self) -> I {
        self.io
    }
    pub fn get_ref(&self) -> &I {
        &self.io
    }
    pub fn get_mut(&mut self) -> &mut I {
        &mut self.io
    }

    pub fn position(&self) -> u64 {
        self.pos
    }
    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<T> Seek for Cursor<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(p) => self.pos = p,
            SeekFrom::Current(p) => {
                let pos = self.pos as i64 + p;
                if pos < 0 {
                    return Err(Error::new(ErrorKind::InvalidInput, "seek to a negative position"));
                }
                self.pos = pos as u64;
            }
            SeekFrom::End(_) => {
                return Err(Error::new(ErrorKind::InvalidInput, "unknown cursor size"))
            }
        };
        Ok(self.pos)
    }
}

impl<T> Read for Cursor<T>
    where T: ReadAt
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = try!(self.get_ref().read_at(buf, self.pos));
        self.pos += bytes as u64;
        Ok(bytes)
    }
}
impl<T> Write for Cursor<T>
    where T: WriteAt
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let pos = self.pos;
        let bytes = try!(self.get_mut().write_at(buf, pos));
        self.pos += bytes as u64;
        Ok(bytes)
    }
    fn flush(&mut self) -> Result<()> {
        WriteAt::flush(self.get_mut())
    }
}


pub struct SizeCursor<I: Size>(Cursor<I>);

// Rust doesn't let us implement both the Size and non-Size cases.
// Wait for RFC 1210 to land.

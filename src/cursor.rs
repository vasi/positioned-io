use std::io::{Result, Read, Write, Seek, SeekFrom, Error, ErrorKind};
use std::ops::{Deref, DerefMut};
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

impl<I> Seek for Cursor<I> {
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
                return Err(Error::new(ErrorKind::InvalidInput, "seek from unknown end"))
            }
        };
        Ok(self.pos)
    }
}

impl<I> Read for Cursor<I>
    where I: ReadAt
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = try!(self.get_ref().read_at(buf, self.pos));
        self.pos += bytes as u64;
        Ok(bytes)
    }
}
impl<I> Write for Cursor<I>
    where I: WriteAt
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

// Rust doesn't let us implement seek for both the Size and non-Size cases.
// Wait for RFC 1210 to land, in the meantime use this.
pub struct SizeCursor<I: Size>(Cursor<I>);
impl<I> SizeCursor<I> where I: Size {
    pub fn new_pos(io: I, pos: u64) -> Self {
        SizeCursor(Cursor::new_pos(io, pos))
    }
    pub fn new(io: I) -> Self {
        SizeCursor(Cursor::new(io))
    }
}

// Automatically fall back to Cursor.
impl<I> Deref for SizeCursor<I> where I: Size {
    type Target = Cursor<I>;
    fn deref(&self) -> &Cursor<I> {
        &self.0
    }
}
impl<I> DerefMut for SizeCursor<I> where I: Size {
    fn deref_mut(&mut self) -> &mut Cursor<I> {
        &mut self.0
    }
}

// We know how to seek from the end for SizeCursor.
impl<I> Seek for SizeCursor<I> where I: Size {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let pos = match pos {
            SeekFrom::Start(p) => p as i64,
            SeekFrom::Current(p) => self.pos as i64 + p,
            SeekFrom::End(p) => match self.get_ref().size() {
                Err(e) => return Err(e),
                Ok(None) => return Err(Error::new(ErrorKind::InvalidData, "seek from unknown end")),
                Ok(Some(s)) => s as i64 + p,
            }
        };
        self.0.pos = pos as u64;
        Ok(self.pos)
    }
}

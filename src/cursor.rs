use std::io::{self, Result, Read, Write, Seek, SeekFrom};
use ::{ReadAt, WriteAt};

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

    fn into_inner(self) -> I {
        self.io
    }
    fn get_ref(&self) -> &I {
        &self.io
    }
    fn get_mut(&mut self) -> &mut I {
        &mut self.io
    }

    fn position(&self) -> u64 {
        self.pos
    }
    fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<T> Seek for Cursor<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        unimplemented!()
    }
}

impl<T> Read for Cursor<T> where T: ReadAt {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // let pos = self.0.position();
        // let result = self.0.get_ref().read_at(buf, pos);
        // self.0.set_position(pos + buf.len());
        // result;
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

use std::io::Result;
use super::{ReadAt, WriteAt, Size};

impl<'a, R: ReadAt + ?Sized> ReadAt for &'a R {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        (**self).read_at(pos, buf)
    }
}

impl<'a, R: ReadAt + ?Sized> ReadAt for &'a mut R {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        (**self).read_at(pos, buf)
    }
}

impl<'a, W: WriteAt + ?Sized> WriteAt for &'a mut W {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> Result<usize> {
        (**self).write_at(pos, buf)
    }
    fn flush(&mut self) -> Result<()> {
        (**self).flush()
    }
}

impl<'a, S: Size + ?Sized> Size for &'a S {
    fn size(&self) -> Result<Option<u64>> {
        (**self).size()
    }
}

impl<'a, S: Size + ?Sized> Size for &'a mut S {
    fn size(&self) -> Result<Option<u64>> {
        (**self).size()
    }
}

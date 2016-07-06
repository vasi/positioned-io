use std::cell::RefCell;
use std::io::Result;
use super::{ReadAt, WriteAt, Size};

impl<'a, R: ReadAt + ?Sized> ReadAt for &'a R {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        R::read_at(self, pos, buf)
    }
}
impl<'a, R: ReadAt + ?Sized> ReadAt for &'a mut R {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        R::read_at(self, pos, buf)
    }
}

impl<'a, W: WriteAt + ?Sized> WriteAt for &'a mut W {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> Result<usize> {
        W::write_at(self, pos, buf)
    }
    fn flush(&mut self) -> Result<()> {
        W::flush(self)
    }
}

impl<'a, S: Size + ?Sized> Size for &'a S {
    fn size(&self) -> Result<Option<u64>> {
        S::size(self)
    }
}
impl<'a, S: Size + ?Sized> Size for &'a mut S {
    fn size(&self) -> Result<Option<u64>> {
        S::size(self)
    }
}

impl<'a, R> ReadAt for &'a RefCell<R> where R: ReadAt {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        self.borrow().read_at(pos, buf)
    }
}
impl<'a, W> WriteAt for &'a RefCell<W> where W: WriteAt {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> Result<usize> {
        self.borrow_mut().write_at(pos, buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.borrow_mut().flush()
    }
}
impl<'a, S> Size for &'a RefCell<S> where S: Size {
    fn size(&self) -> Result<Option<u64>> {
        self.borrow().size()
    }
}

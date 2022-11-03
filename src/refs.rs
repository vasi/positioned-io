use std::{cell::RefCell, io};

use super::{ReadAt, Size, WriteAt};

impl<'a, R: ReadAt + ?Sized> ReadAt for &'a R {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        R::read_at(self, pos, buf)
    }
}

impl<'a, R: ReadAt + ?Sized> ReadAt for &'a mut R {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        R::read_at(self, pos, buf)
    }
}

impl<'a, W: WriteAt + ?Sized> WriteAt for &'a mut W {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> io::Result<usize> {
        W::write_at(self, pos, buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        W::flush(self)
    }
}

impl<'a, S: Size + ?Sized> Size for &'a S {
    fn size(&self) -> io::Result<Option<u64>> {
        S::size(self)
    }
}
impl<'a, S: Size + ?Sized> Size for &'a mut S {
    fn size(&self) -> io::Result<Option<u64>> {
        S::size(self)
    }
}

impl<'a, R: ReadAt> ReadAt for &'a RefCell<R> {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        self.borrow().read_at(pos, buf)
    }
}

impl<'a, W: WriteAt> WriteAt for &'a RefCell<W> {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> io::Result<usize> {
        self.borrow_mut().write_at(pos, buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.borrow_mut().flush()
    }
}

impl<'a, S: Size> Size for &'a RefCell<S> {
    fn size(&self) -> io::Result<Option<u64>> {
        self.borrow().size()
    }
}

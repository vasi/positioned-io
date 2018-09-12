use super::{ReadAt, WriteAt};
use std::fs::File;
use std::io;
use std::io::Write;
use std::os::unix::fs::FileExt;

impl ReadAt for File {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        FileExt::read_at(self, buf, pos)
    }
}

impl WriteAt for File {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> io::Result<usize> {
        FileExt::write_at(self, buf, pos)
    }

    fn flush(&mut self) -> io::Result<()> {
        Write::flush(self)
    }
}

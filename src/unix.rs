use std::fs::File;
use std::io::{Result, Write};
use std::os::unix::io::AsRawFd;
use super::{ReadAt, WriteAt};

extern crate nix;
use self::nix::sys::uio;

impl ReadAt for File {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        let fd = self.as_raw_fd();
        uio::pread(fd, buf, pos as i64).map_err(From::from)
    }
}

impl WriteAt for File {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> Result<usize> {
        let fd = self.as_raw_fd();
        uio::pwrite(fd, buf, pos as i64).map_err(From::from)
    }
    fn flush(&mut self) -> Result<()> {
        Write::flush(self)
    }
}

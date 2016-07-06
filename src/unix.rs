use std::fs::File;
use std::io::{Result, Write};
use std::os::unix::io::AsRawFd;
use ::{ReadAt, WriteAt, Size};

extern crate nix;
use self::nix::sys::uio;
use self::nix::sys::stat::{fstat, SFlag, S_IFREG};

impl ReadAt for File {
    fn read_at(&self, buf: &mut [u8], pos: u64) -> Result<usize> {
        let fd = self.as_raw_fd();
        uio::pread(fd, buf, pos as i64).map_err(From::from)
    }
}

impl WriteAt for File {
    fn write_at(&mut self, buf: &[u8], pos: u64) -> Result<usize> {
        let fd = self.as_raw_fd();
        uio::pwrite(fd, buf, pos as i64).map_err(From::from)
    }
    fn flush(&mut self) -> Result<()> {
        Write::flush(self)
    }
}

impl Size for File {
    fn size(&self) -> Result<Option<u64>> {
        let fd = self.as_raw_fd();
        let stat = try!(fstat(fd));

        // Only regular files have a size.
        if SFlag::from_bits_truncate(stat.st_mode).contains(S_IFREG) {
            return Ok(Some(stat.st_size as u64));
        }
        Ok(None)
    }
}

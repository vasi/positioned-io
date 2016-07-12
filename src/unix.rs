use std::fs::File;
use std::io::{Result, Write, Error};
use std::os::unix::io::AsRawFd;
use super::{ReadAt, WriteAt};

extern crate libc;
use self::libc::{pread, pwrite, c_void, off_t, size_t, ssize_t};

fn err(e: ssize_t) -> Result<usize> {
    if e == -1 as ssize_t {
        Err(Error::last_os_error())
    } else {
        Ok(e as usize)
    }
}

impl ReadAt for File {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        let fd = self.as_raw_fd();
        err(unsafe {
            pread(fd, buf.as_mut_ptr() as *mut c_void, buf.len() as size_t, pos as off_t)
        })
    }
}

impl WriteAt for File {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> Result<usize> {
        let fd = self.as_raw_fd();
        err(unsafe {
            pwrite(fd, buf.as_ptr() as *const c_void, buf.len() as size_t, pos as off_t)
        })
    }
    fn flush(&mut self) -> Result<()> {
        Write::flush(self)
    }
}

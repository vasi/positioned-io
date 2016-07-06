use std::fs::File;
use std::io::{Result, Error, Write};
use std::ptr::null_mut;

use super::{ReadAt, WriteAt};

use std::os::windows::io::AsRawHandle;

extern crate kernel32;
extern crate winapi;
use self::winapi::{BOOL, DWORD, LPVOID};
use self::winapi::minwinbase::OVERLAPPED;
use self::kernel32::ReadFile;

fn result(e: BOOL) -> Result<()> {
    if e == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

fn overlapped(pos: u64) -> OVERLAPPED {
    OVERLAPPED {
        Internal: 0,
        InternalHigh: 0,
        Offset: pos as u32,
        OffsetHigh: (pos >> 32) as u32,
        hEvent: null_mut(),
    }
}

impl ReadAt for File {
    fn read_at(&self, buf: &mut [u8], pos: u64) -> Result<usize> {
        let mut bytes : DWORD = 0;
        let mut ov = overlapped(pos);
        try!(result(unsafe {
            ReadFile(
                self.as_raw_handle(),
                buf.as_mut_ptr() as LPVOID,
                buf.len() as DWORD,
                &mut bytes,
                &mut ov
            )
        }));
        Ok(bytes as usize)
    }
}

impl WriteAt for File {
    fn write_at(&mut self, buf: &[u8], pos: u64) -> Result<usize> {
        let mut bytes : DWORD = 0;
        let mut ov = overlapped(pos);
        try!(result(unsafe {
            ReadFile(
                self.as_raw_handle(),
                buf.as_ptr() as LPVOID,
                buf.len() as DWORD,
                &mut bytes,
                &mut ov
            )
        }));
        Ok(bytes as usize)
    }
    fn flush(&mut self) -> Result<()> {
        Write::flush(self)
    }
}

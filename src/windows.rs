use std::fs::File;
use std::io::{Result, Error, Write};
use std::mem;
use std::ptr;

use super::{ReadAt, WriteAt};

use std::os::windows::io::AsRawHandle;

extern crate winapi;
use self::winapi::shared::minwindef::{BOOL, DWORD, LPVOID};
use self::winapi::um::winnt::HANDLE;
use self::winapi::um::minwinbase::{OVERLAPPED, OVERLAPPED_u, OVERLAPPED_u_s};
use self::winapi::um::fileapi::ReadFile;

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
        u: unsafe {
            let mut u: OVERLAPPED_u = mem::zeroed();
            *u.s_mut() = OVERLAPPED_u_s {
                Offset: pos as u32,
                OffsetHigh: (pos >> 32) as u32,
            };
            u
        },
        hEvent: ptr::null_mut(),
    }
}

impl ReadAt for File {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        let mut bytes: DWORD = 0;
        let mut ov = overlapped(pos);
        result(unsafe {
            ReadFile(self.as_raw_handle() as HANDLE,
                     buf.as_mut_ptr() as LPVOID,
                     buf.len() as DWORD,
                     &mut bytes,
                     &mut ov)
        })?;
        Ok(bytes as usize)
    }
}

impl WriteAt for File {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> Result<usize> {
        let mut bytes: DWORD = 0;
        let mut ov = overlapped(pos);
        result(unsafe {
            ReadFile(self.as_raw_handle() as HANDLE,
                     buf.as_ptr() as LPVOID,
                     buf.len() as DWORD,
                     &mut bytes,
                     &mut ov)
        })?;
        Ok(bytes as usize)
    }
    fn flush(&mut self) -> Result<()> {
        Write::flush(self)
    }
}

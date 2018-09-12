use std::fs::File;
use std::io;
use std::io::Write;
use std::mem;
use std::ptr;
use std::cmp::min;

use super::{ReadAt, WriteAt};

use std::os::windows::io::AsRawHandle;

extern crate winapi;
use self::winapi::shared::basetsd::SIZE_T;
use self::winapi::shared::minwindef::{BOOL, DWORD, LPVOID};
use self::winapi::um::fileapi::ReadFile;
use self::winapi::um::handleapi::CloseHandle;
use self::winapi::um::minwinbase::{OVERLAPPED_u, OVERLAPPED_u_s, OVERLAPPED};
use self::winapi::um::sysinfoapi::GetSystemInfo;
use self::winapi::um::winnt::{HANDLE, PAGE_READONLY};
use self::winapi::um::memoryapi::{CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, FILE_MAP_READ};

fn result(e: BOOL) -> io::Result<()> {
    if e == 0 {
        Err(io::Error::last_os_error())
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

fn allocation_granularity() -> u64 {
    unsafe {
        let mut info = mem::zeroed();
        GetSystemInfo(&mut info);
        info.dwAllocationGranularity as u64
    }
}

impl ReadAt for File {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let file_len = self.metadata()?.len();
        if (usize::max_value() as u64) < file_len {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "file length overflows usize"));
        }

        let len = min(file_len, pos + buf.len() as u64) - pos;

        unsafe {
            let alignment = pos % allocation_granularity();
            let aligned_pos = pos - alignment;
            let aligned_len = len + alignment;

            let mapping = CreateFileMappingW(
                self.as_raw_handle() as HANDLE,
                ptr::null_mut(),
                PAGE_READONLY,
                0,
                0,
                ptr::null(),
            );

            if mapping.is_null() {
                return Err(io::Error::last_os_error());
            }

            let aligned_ptr = MapViewOfFile(
                mapping,
                FILE_MAP_READ,
                (aligned_pos >> 32) as DWORD,
                (aligned_pos & 0xffff_ffff) as DWORD,
                aligned_len as SIZE_T,
            );

            CloseHandle(mapping);

            if aligned_ptr.is_null() {
                return Err(io::Error::last_os_error());
            }

            let ptr = (aligned_ptr as *const u8).offset(alignment as isize);
            ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), len as usize);

            if 0 == UnmapViewOfFile(aligned_ptr) {
                return Err(io::Error::last_os_error());
            }
        }

        Ok(len as usize)
    }
}

impl WriteAt for File {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> io::Result<usize> {
        let mut bytes: DWORD = 0;
        let mut ov = overlapped(pos);
        result(unsafe {
            ReadFile(
                self.as_raw_handle() as HANDLE,
                buf.as_ptr() as LPVOID,
                buf.len() as DWORD,
                &mut bytes,
                &mut ov,
            )
        })?;
        Ok(bytes as usize)
    }

    fn flush(&mut self) -> io::Result<()> {
        Write::flush(self)
    }
}

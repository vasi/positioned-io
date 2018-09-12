use std::fs::File;
use std::io;
use std::io::{Write, Seek, SeekFrom};
use std::mem;
use std::ptr;
use std::cmp::min;
use std::os::windows::fs::FileExt;

use super::{ReadAt, WriteAt};

use std::os::windows::io::AsRawHandle;

extern crate winapi;
use self::winapi::shared::basetsd::SIZE_T;
use self::winapi::shared::minwindef::{BOOL, DWORD};
use self::winapi::um::handleapi::CloseHandle;
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

fn allocation_granularity() -> u64 {
    unsafe {
        let mut info = mem::zeroed();
        GetSystemInfo(&mut info);
        info.dwAllocationGranularity as u64
    }
}

impl ReadAt for File {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        let file_len = self.metadata()?.len();
        if (usize::max_value() as u64) < file_len {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "file length overflows usize"));
        }

        if buf.is_empty() || pos >= file_len {
            return Ok(0);
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

            result(UnmapViewOfFile(aligned_ptr))?;
        }

        Ok(len as usize)
    }
}

impl WriteAt for File {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> io::Result<usize> {
        let cursor = self.seek(SeekFrom::Current(0))?;
        let result = self.seek_write(buf, pos)?;
        self.seek(SeekFrom::Start(cursor))?;
        Ok(result)
    }

    fn flush(&mut self) -> io::Result<()> {
        Write::flush(self)
    }
}

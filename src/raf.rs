use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
#[cfg(unix)]
use std::os::unix::fs::FileExt;
#[cfg(windows)]
use std::os::windows::fs::FileExt;

use super::{ReadAt, WriteAt};

#[derive(Debug)]
pub struct RandomAccessFile {
    file: File,
    #[cfg(not(unix))]
    pos: u64,
}

impl RandomAccessFile {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<RandomAccessFile> {
        RandomAccessFile::try_new(File::open(path)?)
    }

    pub fn try_new(file: File) -> io::Result<RandomAccessFile> {
        RandomAccessFile::try_new_impl(file)
    }

    #[cfg(unix)]
    fn try_new_impl(file: File) -> io::Result<RandomAccessFile> {
        unsafe {
            use std::os::unix::io::AsRawFd;
            libc::posix_fadvise(file.as_raw_fd(), 0, file.metadata()?.len() as i64, libc::POSIX_FADV_RANDOM);
        }

        Ok(RandomAccessFile { file })
    }

    #[cfg(not(unix))]
    fn try_new_impl(file: File) -> io::Result<RandomAccessFile> {
        Ok(RandomAccessFile {
            file,
            pos: file.seek(SeekFrom::Current(0))?,
        })
    }

    pub fn try_into_inner(self) -> Result<File, (RandomAccessFile, io::Error)> {
        RandomAccessFile::try_into_inner_impl(self)
    }

    #[cfg(unix)]
    fn try_into_inner_impl(self) -> Result<File, (RandomAccessFile, io::Error)> {
        Ok(self.file)
    }

    #[cfg(not(unix))]
    fn try_into_inner_impl(self) -> Result<File, (RandomAccessFile, io::Error)> {
        match self.file.seek(SeekFrom::Start(self.pos)) {
            Ok(_) => Ok(self.file),
            Err(err) => Err((self, err)),
        }
    }
}


#[cfg(unix)]
impl ReadAt for RandomAccessFile {
    #[inline]
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        FileExt::read_at(&self.file, buf, pos)
    }
}

#[cfg(unix)]
impl WriteAt for RandomAccessFile {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> io::Result<usize> {
        FileExt::write_at(&self.file, buf, pos)
    }

    fn flush(&mut self) -> io::Result<()> {
        Write::flush(&mut self.file)
    }
}

#[cfg(windows)]
impl ReadAt for RandomAccessFile {
    #[inline]
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        FileExt::seek_read(&self.file, pos, buf)
    }
}

#[cfg(windows)]
impl WriteAt for RandomAccessFile {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> io::Result<usize> {
        FileExt::seek_write(&self.file, buf, pos)
    }

    fn flush(&mut self) -> io::Result<()> {
        Write::flush(&mut self.file)
    }
}

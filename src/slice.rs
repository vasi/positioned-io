use super::{ReadAt, WriteAt, Size};

use std::cmp::min;
use std::io::{Result, Error, ErrorKind};

/// A window into another `ReatAt` or `WriteAt`.
///
/// Given an existing positioned I/O, this presents a limited view of it.
///
/// # Examples
///
/// Some slices have size restrictions:
///
/// ```rust
/// # use std::io;
/// use positioned_io::{ReadAt, Slice};
///
/// # fn foo() -> io::Result<()> {
/// let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let slice = Slice::new(a.as_ref(), 4, Some(4));
///
/// let mut buf = [0; 4];
/// let bytes = try!(slice.read_at(2, &mut buf));
/// assert_eq!(bytes, 2);
/// assert_eq!(buf, [6, 7, 0, 0]);
/// # Ok(())
/// # }
/// # fn main() { foo().unwrap(); }
/// ```
///
/// Some slices do not:
///
/// ```rust
/// # use std::io;
/// use positioned_io::{WriteAt, Slice};
///
/// # fn foo() -> io::Result<()> {
/// let mut v = vec![0, 1, 2, 3, 4, 5];
/// let buf = [9; 3];
/// {
///     let mut slice = Slice::new(&mut v, 2, None);
///     try!(slice.write_all_at(3, &buf));
/// }
/// // The write goes right past the end.
/// assert_eq!(v, vec![0, 1, 2, 3, 4, 9, 9, 9]);
/// # Ok(())
/// # }
/// # fn main() { foo().unwrap(); }
/// ```
pub struct Slice<I> {
    io: I,
    offset: u64,
    size: Option<u64>,
}

impl<I> Slice<I> {
    /// Create a new `Slice`.
    ///
    /// The slice will be a view of `size` bytes, starting at `offset` in `io`. If you don't
    /// pass a size, the size won't be limited.
    pub fn new(io: I, offset: u64, size: Option<u64>) -> Self {
        Slice {
            io,
            offset,
            size,
        }
    }

    // Get the available bytes starting at some point.
    fn avail(&self, pos: u64, bytes: usize) -> usize {
        match self.size {
            None => bytes,
            Some(size) if pos >= size => 0,
            Some(size) => min(bytes as u64, size - pos) as usize,
        }
    }
}
impl<I> Slice<I>
    where I: Size
{
    /// Create a new `Slice` that goes to the end of `io`.
    ///
    /// Note that you can create a larger slice by passing a larger size to `new()`, but it won't
    /// do you any good for reading.
    pub fn new_to_end(io: I, offset: u64) -> Result<Self> {
        match io.size() {
            Ok(Some(size)) => Ok(Self::new(io, offset, Some(size - offset))),
            _ => Err(Error::new(ErrorKind::InvalidData, "unknown base size")),
        }
    }
}

impl<I> ReadAt for Slice<I>
    where I: ReadAt
{
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.avail(pos, buf.len());
        self.io.read_at(pos + self.offset, &mut buf[..bytes])
    }
}

impl<I> WriteAt for Slice<I>
    where I: WriteAt
{
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> Result<usize> {
        let bytes = self.avail(pos, buf.len());
        self.io.write_at(pos + self.offset, &buf[..bytes])
    }

    fn flush(&mut self) -> Result<()> {
        self.io.flush()
    }
}

impl<I> Size for Slice<I> {
    fn size(&self) -> Result<Option<u64>> {
        Ok(self.size)
    }
}

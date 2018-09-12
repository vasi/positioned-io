use std::io::{Result, Read, Write, Seek, SeekFrom, Error, ErrorKind};
use std::ops::{Deref, DerefMut};
use super::{ReadAt, WriteAt, Size};

/// Adapts a `ReadAt` or `WriteAt` into a `Read` or `Write`.
///
/// This wraps anything that read and write at offsets, turning into an object that can
/// read or write at a file position. This allows you to use those types with all the many
/// functions that expect a `Read` or `Write`.
///
/// Note that seeking on `Cursor` has limited functionality. We don't know how many bytes are
/// available, so we can't use SeekFrom::End. See [`SizeCursor`][SizeCursor] for another option.
///
/// [SizeCursor]: struct.SizeCursor.html
///
/// # Examples
///
/// ```no_run
/// # use std::io::{self, Result, Read};
/// # use std::fs::File;
/// use positioned_io::{ReadAt, Cursor};
///
/// struct NetworkStorage {
///     // A remote disk that supports random access.
/// }
/// # impl NetworkStorage {
/// #   fn new(i: i32) -> Self { NetworkStorage { } }
/// # }
/// impl ReadAt for NetworkStorage {
///     // ...
/// #   fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
/// #       Ok(0)
/// #   }
/// }
///
/// # const SOME_LOCATION: i32 = 1;
/// # fn foo() -> Result<()> {
/// // Adapt our storage into a Read.
/// let storage = NetworkStorage::new(SOME_LOCATION);
/// let curs = Cursor::new_pos(storage, 1 << 30);
///
/// // Copy a segment to a file.
/// let mut input = curs.take(1 << 20);
/// let mut output = File::create("segment.out")?;
/// io::copy(&mut input, &mut output)?;
/// # Ok(())
/// # }
/// ```
pub struct Cursor<I> {
    io: I,
    pos: u64,
}
impl<I> Cursor<I> {
    /// Create a new `Cursor` which starts reading at a specified offset.
    ///
    /// Pass in a `ReadAt` or `WriteAt` as `io`.
    pub fn new_pos(io: I, pos: u64) -> Self {
        Cursor { io, pos }
    }
    /// Create a new Cursor which starts reading at offset zero.
    ///
    /// Pass in a `ReadAt` or `WriteAt` as `io`.
    pub fn new(io: I) -> Self {
        Self::new_pos(io, 0)
    }

    /// Consume `self` and yield the inner `ReadAt` or `WriteAt`.
    pub fn into_inner(self) -> I {
        self.io
    }
    /// Borrow the inner `ReadAt` or `WriteAt`.
    pub fn get_ref(&self) -> &I {
        &self.io
    }
    /// Borrow the inner `ReadAt` or `WriteAt` mutably.
    pub fn get_mut(&mut self) -> &mut I {
        &mut self.io
    }

    /// Get the current read/write position.
    pub fn position(&self) -> u64 {
        self.pos
    }
    /// Set the current read/write position.
    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<I> Seek for Cursor<I> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(p) => self.pos = p,
            SeekFrom::Current(p) => {
                let pos = self.pos as i64 + p;
                if pos < 0 {
                    return Err(Error::new(ErrorKind::InvalidInput, "seek to a negative position"));
                }
                self.pos = pos as u64;
            }
            SeekFrom::End(_) => {
                return Err(Error::new(ErrorKind::InvalidInput, "seek from unknown end"))
            }
        };
        Ok(self.pos)
    }
}

impl<I> Read for Cursor<I>
    where I: ReadAt
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.get_ref().read_at(self.pos, buf)?;
        self.pos += bytes as u64;
        Ok(bytes)
    }
}
impl<I> Write for Cursor<I>
    where I: WriteAt
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let pos = self.pos;
        let bytes = self.get_mut().write_at(pos, buf)?;
        self.pos += bytes as u64;
        Ok(bytes)
    }
    fn flush(&mut self) -> Result<()> {
        WriteAt::flush(self.get_mut())
    }
}

/// Adapts a `ReadAt` or `WriteAt` into a `Read` or `Write`, with better seeking.
///
/// This is just like [`Cursor`][Cursor], except that it requires an object that implements
/// [`Size`][Size], and that it can seek from the end of the I/O object.
///
/// Eventually it will be legal to specialize `Cursor` for types that implement `Size`, see
/// [RFC 1210][RFC].
///
/// [Cursor]: struct.Cursor.html
/// [Size]: trait.Size.html
/// [RFC]: https://github.com/rust-lang/rfcs/pull/1210
pub struct SizeCursor<I: Size>(Cursor<I>);
impl<I> SizeCursor<I>
    where I: Size
{
    /// Create a new `SizeCursor` which starts reading at a specified offset.
    ///
    /// Pass in a `ReadAt` or `WriteAt` as `io`.
    pub fn new_pos(io: I, pos: u64) -> Self {
        SizeCursor(Cursor::new_pos(io, pos))
    }
    /// Create a new `SizeCursor` which starts reading at offset zero.
    ///
    /// Pass in a `ReadAt` or `WriteAt` as `io`.
    pub fn new(io: I) -> Self {
        SizeCursor(Cursor::new(io))
    }
}

// Automatically fall back to Cursor.
impl<I> Deref for SizeCursor<I>
    where I: Size
{
    type Target = Cursor<I>;
    fn deref(&self) -> &Cursor<I> {
        &self.0
    }
}
impl<I> DerefMut for SizeCursor<I>
    where I: Size
{
    fn deref_mut(&mut self) -> &mut Cursor<I> {
        &mut self.0
    }
}

// We know how to seek from the end for SizeCursor.
impl<I> Seek for SizeCursor<I>
    where I: Size
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let pos = match pos {
            SeekFrom::Start(p) => p as i64,
            SeekFrom::Current(p) => self.pos as i64 + p,
            SeekFrom::End(p) => {
                match self.get_ref().size() {
                    Err(e) => return Err(e),
                    Ok(None) => {
                        return Err(Error::new(ErrorKind::InvalidData, "seek from unknown end"))
                    }
                    Ok(Some(s)) => s as i64 + p,
                }
            }
        };
        self.0.pos = pos as u64;
        Ok(self.pos)
    }
}

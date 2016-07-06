use extbyteorder::ByteOrder;
use extbyteorder::ReadBytesExt as ExtReadBytesExt;
use extbyteorder::WriteBytesExt as ExtWriteBytesExt;

use std::marker::PhantomData;
use std::io::{Result, Read, Write};
use std::ops::{Deref, DerefMut};

use super::super::{ReadAt, WriteAt};
use super::{ReadBytesExt, WriteBytesExt};

/// Read or write with a given inherent byte-order.
///
/// If you know that you'll always be using a single endianness, an instance of `ByteIo` will
/// allow you to omit the endian specifier on every read or write.
///
/// # Examples
///
/// ```rust
/// # extern crate positioned_io;
/// # extern crate byteorder;
/// # use std::io;
/// # use byteorder::BigEndian;
/// use positioned_io::ByteIo;
///
/// # fn foo() -> io::Result<()> {
/// let mut buf = [0; 8];
/// {
///     let mut io : ByteIo<_, BigEndian> = ByteIo::new(buf.as_mut());
///     // All writes will automatically be BigEndian.
///     try!(io.write_u16(300));
///     try!(io.write_u32(1_000_000));
///     try!(io.write_i16(-1));
///  }
/// assert_eq!(buf, [1, 44, 0, 15, 66, 64, 255, 255]);
/// # Ok(())
/// # }
/// # fn main() { foo().unwrap() }
/// ```
pub struct ByteIo<I, E: ByteOrder> {
    io: I,
    endianness: PhantomData<E>,
}

impl<I, E> ByteIo<I, E>
    where E: ByteOrder
{
    /// Create a new `ByteIo` from some sort of reader or writer.
    ///
    /// You will need to specify the byte-order when creating a ByteIo.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate positioned_io;
    /// # extern crate byteorder;
    /// # use byteorder::BigEndian;
    /// use positioned_io::ByteIo;
    ///
    /// # fn main() {
    /// let buf = [0; 10];
    /// // Add a type specifier for the byte order.
    /// let io : ByteIo<_, BigEndian> = ByteIo::new(buf.as_ref());
    /// # }
    /// ```
    pub fn new(io: I) -> Self {
        ByteIo {
            io: io,
            endianness: PhantomData,
        }
    }
}

// Auto-coerce back to the base IO.
impl<I, E> Deref for ByteIo<I, E>
    where E: ByteOrder
{
    type Target = I;
    fn deref(&self) -> &I {
        &self.io
    }
}
impl<I, E> DerefMut for ByteIo<I, E>
    where E: ByteOrder
{
    fn deref_mut(&mut self) -> &mut I {
        &mut self.io
    }
}

impl<I, E: ByteOrder> ByteIo<I, E>
    where I: Read
{
    /// Reads an unsigned 16-bit integer.
    pub fn read_u16(&mut self) -> Result<u16> {
        self.io.read_u16::<E>()
    }
    /// Reads a signed 16-bit integer.
    pub fn read_i16(&mut self) -> Result<i16> {
        self.io.read_i16::<E>()
    }
    /// Reads an unsigned 32-bit integer.
    pub fn read_u32(&mut self) -> Result<u32> {
        self.io.read_u32::<E>()
    }
    /// Reads a signed 32-bit integer.
    pub fn read_i32(&mut self) -> Result<i32> {
        self.io.read_i32::<E>()
    }
    /// Reads an unsigned 64-bit integer.
    pub fn read_u64(&mut self) -> Result<u64> {
        self.io.read_u64::<E>()
    }
    /// Reads a signed 64-bit integer.
    pub fn read_i64(&mut self) -> Result<i64> {
        self.io.read_i64::<E>()
    }
    /// Reads an unsigned `nbytes`-bit integer.
    pub fn read_uint(&mut self, nbytes: usize) -> Result<u64> {
        self.io.read_uint::<E>(nbytes)
    }
    /// Reads a signed `nbytes`-bit integer.
    pub fn read_int(&mut self, nbytes: usize) -> Result<i64> {
        self.io.read_int::<E>(nbytes)
    }
    /// Reads a single-precision floating point number.
    pub fn read_f32(&mut self) -> Result<f32> {
        self.io.read_f32::<E>()
    }
    /// Reads a double-precision floating point number.
    pub fn read_f64(&mut self) -> Result<f64> {
        self.io.read_f64::<E>()
    }
}
impl<I, E: ByteOrder> ByteIo<I, E>
    where I: Write
{
    /// Writes an unsigned 16-bit integer.
    pub fn write_u16(&mut self, n: u16) -> Result<()> {
        self.io.write_u16::<E>(n)
    }
    /// Writes a signed 16-bit integer.
    pub fn write_i16(&mut self, n: i16) -> Result<()> {
        self.io.write_i16::<E>(n)
    }
    /// Writes an unsigned 32-bit integer.
    pub fn write_u32(&mut self, n: u32) -> Result<()> {
        self.io.write_u32::<E>(n)
    }
    /// Writes a signed 32-bit integer.
    pub fn write_i32(&mut self, n: i32) -> Result<()> {
        self.io.write_i32::<E>(n)
    }
    /// Writes an unsigned 64-bit integer.
    pub fn write_u64(&mut self, n: u64) -> Result<()> {
        self.io.write_u64::<E>(n)
    }
    /// Writes a signed 64-bit integer.
    pub fn write_i64(&mut self, n: i64) -> Result<()> {
        self.io.write_i64::<E>(n)
    }
    /// Writes an unsigned `nbytes`-bit integer.
    pub fn write_uint(&mut self, n: u64, nbytes: usize) -> Result<()> {
        self.io.write_uint::<E>(n, nbytes)
    }
    /// Writes a signed `nbytes`-bit integer.
    pub fn write_int(&mut self, n: i64, nbytes: usize) -> Result<()> {
        self.io.write_int::<E>(n, nbytes)
    }
    /// Writes a single-precision floating point number.
    pub fn write_f32(&mut self, n: f32) -> Result<()> {
        self.io.write_f32::<E>(n)
    }
    /// Writes a double-precision floating point number.
    pub fn write_f64(&mut self, n: f64) -> Result<()> {
        self.io.write_f64::<E>(n)
    }
}
impl<I, E: ByteOrder> ByteIo<I, E>
    where I: ReadAt
{
    /// Reads an unsigned 16-bit integer at an offset.
    pub fn read_u16_at(&self, pos: u64) -> Result<u16> {
        self.io.read_u16_at::<E>(pos)
    }
    /// Reads a signed 16-bit integer at an offset.
    pub fn read_i16_at(&self, pos: u64) -> Result<i16> {
        self.io.read_i16_at::<E>(pos)
    }
    /// Reads an unsigned 32-bit integer at an offset.
    pub fn read_u32_at(&self, pos: u64) -> Result<u32> {
        self.io.read_u32_at::<E>(pos)
    }
    /// Reads a signed 32-bit integer at an offset.
    pub fn read_i32_at(&self, pos: u64) -> Result<i32> {
        self.io.read_i32_at::<E>(pos)
    }
    /// Reads an unsigned 64-bit integer at an offset.
    pub fn read_u64_at(&self, pos: u64) -> Result<u64> {
        self.io.read_u64_at::<E>(pos)
    }
    /// Reads a signed 64-bit integer at an offset.
    pub fn read_i64_at(&self, pos: u64) -> Result<i64> {
        self.io.read_i64_at::<E>(pos)
    }
    /// Reads an unsigned `nbytes`-bit integer at an offset.
    pub fn read_uint_at(&self, pos: u64, nbytes: usize) -> Result<u64> {
        self.io.read_uint_at::<E>(pos, nbytes)
    }
    /// Reads a signed `nbytes`-bit integer at an offset.
    pub fn read_int_at(&self, pos: u64, nbytes: usize) -> Result<i64> {
        self.io.read_int_at::<E>(pos, nbytes)
    }
    /// Reads a single-precision floating point number at an offset.
    pub fn read_f32_at(&self, pos: u64) -> Result<f32> {
        self.io.read_f32_at::<E>(pos)
    }
    /// Reads a double-precision floating point number at an offset.
    pub fn read_f64_at(&self, pos: u64) -> Result<f64> {
        self.io.read_f64_at::<E>(pos)
    }
}
impl<I, E: ByteOrder> ByteIo<I, E>
    where I: WriteAt
{
    /// Writes an unsigned 16-bit integer to an offset.
    pub fn write_u16_at(&mut self, pos: u64, n: u16) -> Result<()> {
        self.io.write_u16_at::<E>(pos, n)
    }
    /// Writes a signed 16-bit integer to an offset.
    pub fn write_i16_at(&mut self, pos: u64, n: i16) -> Result<()> {
        self.io.write_i16_at::<E>(pos, n)
    }
    /// Writes an unsigned 32-bit integer to an offset.
    pub fn write_u32_at(&mut self, pos: u64, n: u32) -> Result<()> {
        self.io.write_u32_at::<E>(pos, n)
    }
    /// Writes a signed 32-bit integer to an offset.
    pub fn write_i32_at(&mut self, pos: u64, n: i32) -> Result<()> {
        self.io.write_i32_at::<E>(pos, n)
    }
    /// Writes an unsigned 64-bit integer to an offset.
    pub fn write_u64_at(&mut self, pos: u64, n: u64) -> Result<()> {
        self.io.write_u64_at::<E>(pos, n)
    }
    /// Writes a signed 64-bit integer to an offset.
    pub fn write_i64_at(&mut self, pos: u64, n: i64) -> Result<()> {
        self.io.write_i64_at::<E>(pos, n)
    }
    /// Writes an unsigned `nbytes`-bit integer to an offset.
    pub fn write_uint_at(&mut self, pos: u64, n: u64, nbytes: usize) -> Result<()> {
        self.io.write_uint_at::<E>(pos, n, nbytes)
    }
    /// Writes a signed `nbytes`-bit integer to an offset.
    pub fn write_int_at(&mut self, pos: u64, n: i64, nbytes: usize) -> Result<()> {
        self.io.write_int_at::<E>(pos, n, nbytes)
    }
    /// Writes a single-precision floating point number to an offset.
    pub fn write_f32_at(&mut self, pos: u64, n: f32) -> Result<()> {
        self.io.write_f32_at::<E>(pos, n)
    }
    /// Writes a double-precision floating point number to an offset.
    pub fn write_f64_at(&mut self, pos: u64, n: f64) -> Result<()> {
        self.io.write_f64_at::<E>(pos, n)
    }
}

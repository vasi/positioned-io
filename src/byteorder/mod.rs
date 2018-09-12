pub mod byteio;

use extbyteorder::ByteOrder;

use std::io::Result;

use super::{ReadAt, WriteAt};

/// Extends `ReadAt` with methods for reading numbers at offsets.
///
/// For most of these methods, you need to explicitly add a `ByteOrder` type parameter. See
/// [`byteorder::ReadBytesExt`][byteorder].
///
/// # Examples
///
/// Read an integer from the middle of a byte array:
///
/// ```rust
/// # extern crate positioned_io;
/// # extern crate byteorder;
/// # use std::io;
/// # use byteorder::BigEndian;
/// use positioned_io::ReadBytesAtExt;
///
/// # fn foo() -> io::Result<()> {
/// let buf = [0, 5, 254, 212, 0, 3];
/// let n = buf.as_ref().read_i16_at::<BigEndian>(2)?;
/// assert_eq!(n, -300);
/// # Ok(())
/// # }
/// # fn main() { foo().unwrap() }
/// ```
///
/// [byteorder]: http://burntsushi.net/rustdoc/byteorder/trait.ReadBytesExt.html
pub trait ReadBytesAtExt: ReadAt {
    /// Reads an unsigned 8-bit integer at an offset.
    fn read_u8_at(&self, pos: u64) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact_at(pos, &mut buf)?;
        Ok(buf[0])
    }
    /// Reads a signed 8-bit integer at an offset.
    fn read_i8_at(&self, pos: u64) -> Result<i8> {
        let mut buf = [0; 1];
        self.read_exact_at(pos, &mut buf)?;
        Ok(buf[0] as i8)
    }
    /// Reads an unsigned 16-bit integer at an offset.
    fn read_u16_at<T: ByteOrder>(&self, pos: u64) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact_at(pos, &mut buf)?;
        Ok(T::read_u16(&buf))
    }
    /// Reads a signed 16-bit integer at an offset.
    fn read_i16_at<T: ByteOrder>(&self, pos: u64) -> Result<i16> {
        let mut buf = [0; 2];
        self.read_exact_at(pos, &mut buf)?;
        Ok(T::read_i16(&buf))
    }
    /// Reads an unsigned 32-bit integer at an offset.
    fn read_u32_at<T: ByteOrder>(&self, pos: u64) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact_at(pos, &mut buf)?;
        Ok(T::read_u32(&buf))
    }
    /// Reads a signed 32-bit integer at an offset.
    fn read_i32_at<T: ByteOrder>(&self, pos: u64) -> Result<i32> {
        let mut buf = [0; 4];
        self.read_exact_at(pos, &mut buf)?;
        Ok(T::read_i32(&buf))
    }
    /// Reads an unsigned 64-bit integer at an offset.
    fn read_u64_at<T: ByteOrder>(&self, pos: u64) -> Result<u64> {
        let mut buf = [0; 8];
        self.read_exact_at(pos, &mut buf)?;
        Ok(T::read_u64(&buf))
    }
    /// Reads a signed 64-bit integer at an offset.
    fn read_i64_at<T: ByteOrder>(&self, pos: u64) -> Result<i64> {
        let mut buf = [0; 8];
        self.read_exact_at(pos, &mut buf)?;
        Ok(T::read_i64(&buf))
    }
    /// Reads an unsigned `nbytes`-bit integer at an offset.
    fn read_uint_at<T: ByteOrder>(&self, pos: u64, nbytes: usize) -> Result<u64> {
        let mut buf = [0; 8];
        self.read_exact_at(pos, &mut buf[..nbytes])?;
        Ok(T::read_uint(&buf[..nbytes], nbytes))
    }
    /// Reads a signed `nbytes`-bit integer at an offset.
    fn read_int_at<T: ByteOrder>(&self, pos: u64, nbytes: usize) -> Result<i64> {
        let mut buf = [0; 8];
        self.read_exact_at(pos, &mut buf[..nbytes])?;
        Ok(T::read_int(&buf[..nbytes], nbytes))
    }
    /// Reads a single-precision floating point number at an offset.
    fn read_f32_at<T: ByteOrder>(&self, pos: u64) -> Result<f32> {
        let mut buf = [0; 4];
        self.read_exact_at(pos, &mut buf)?;
        Ok(T::read_f32(&buf))
    }
    /// Reads a double-precision floating point number at an offset.
    fn read_f64_at<T: ByteOrder>(&self, pos: u64) -> Result<f64> {
        let mut buf = [0; 8];
        self.read_exact_at(pos, &mut buf)?;
        Ok(T::read_f64(&buf))
    }
}

/// Extends `WriteAt` with methods for writing numbers at offsets.
///
/// For most of these methods, you need to explicitly add a `ByteOrder` type parameter. See
/// [`byteorder::WriteBytesExt`][byteorder].
///
/// # Examples
///
/// Write an integer to the middle of a byte array:
///
/// ```rust
/// # extern crate positioned_io;
/// # extern crate byteorder;
/// # use std::io;
/// # use byteorder::BigEndian;
/// use positioned_io::WriteBytesAtExt;
///
/// # fn foo() -> io::Result<()> {
/// let mut buf = [0; 6];
/// buf.as_mut().write_u16_at::<BigEndian>(2, 300)?;
/// assert_eq!(buf, [0, 0, 1, 44, 0, 0]);
/// # Ok(())
/// # }
/// # fn main() { foo().unwrap() }
/// ```
///
/// [byteorder]: http://burntsushi.net/rustdoc/byteorder/trait.WriteBytesExt.html
pub trait WriteBytesAtExt: WriteAt {
    /// Writes an unsigned 8-bit integer to an offset.
    fn write_u8_at(&mut self, pos: u64, n: u8) -> Result<()> {
        self.write_all_at(pos, &[n])
    }
    /// Writes a signed 8-bit integer to an offset.
    fn write_i8_at(&mut self, pos: u64, n: i8) -> Result<()> {
        self.write_all_at(pos, &[n as u8])
    }
    /// Writes an unsigned 16-bit integer to an offset.
    fn write_u16_at<T: ByteOrder>(&mut self, pos: u64, n: u16) -> Result<()> {
        let mut buf = [0; 2];
        T::write_u16(&mut buf, n);
        self.write_all_at(pos, &buf)
    }
    /// Writes a signed 16-bit integer to an offset.
    fn write_i16_at<T: ByteOrder>(&mut self, pos: u64, n: i16) -> Result<()> {
        let mut buf = [0; 2];
        T::write_i16(&mut buf, n);
        self.write_all_at(pos, &buf)
    }
    /// Writes an unsigned 32-bit integer to an offset.
    fn write_u32_at<T: ByteOrder>(&mut self, pos: u64, n: u32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, n);
        self.write_all_at(pos, &buf)
    }
    /// Writes a signed 32-bit integer to an offset.
    fn write_i32_at<T: ByteOrder>(&mut self, pos: u64, n: i32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_i32(&mut buf, n);
        self.write_all_at(pos, &buf)
    }
    /// Writes an unsigned 64-bit integer to an offset.
    fn write_u64_at<T: ByteOrder>(&mut self, pos: u64, n: u64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_u64(&mut buf, n);
        self.write_all_at(pos, &buf)
    }
    /// Writes a signed 64-bit integer to an offset.
    fn write_i64_at<T: ByteOrder>(&mut self, pos: u64, n: i64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_i64(&mut buf, n);
        self.write_all_at(pos, &buf)
    }
    /// Writes an unsigned `nbytes`-bit integer to an offset.
    fn write_uint_at<T: ByteOrder>(&mut self, pos: u64, n: u64, nbytes: usize) -> Result<()> {
        let mut buf = [0; 8];
        T::write_uint(&mut buf, n, nbytes);
        self.write_all_at(pos, &buf[..nbytes])
    }
    /// Writes a signed `nbytes`-bit integer to an offset.
    fn write_int_at<T: ByteOrder>(&mut self, pos: u64, n: i64, nbytes: usize) -> Result<()> {
        let mut buf = [0; 8];
        T::write_int(&mut buf, n, nbytes);
        self.write_all_at(pos, &buf[..nbytes])
    }
    /// Writes a single-precision floating point number to an offset.
    fn write_f32_at<T: ByteOrder>(&mut self, pos: u64, n: f32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_f32(&mut buf, n);
        self.write_all_at(pos, &buf)
    }
    /// Writes a double-precision floating point number to an offset.
    fn write_f64_at<T: ByteOrder>(&mut self, pos: u64, n: f64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_f64(&mut buf, n);
        self.write_all_at(pos, &buf)
    }
}

// Implement for everything that does positioned IO.
impl<R: ReadAt> ReadBytesAtExt for R {}
impl<W: WriteAt> WriteBytesAtExt for W {}

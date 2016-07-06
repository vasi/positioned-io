pub mod byteio;

use extbyteorder::ByteOrder;

use std::io::Result;

use super::{ReadAt, WriteAt};

// Read/write bytes at positions.
pub trait ReadBytesExt: ReadAt {
    fn read_u8_at(&self, pos: u64) -> Result<u8> {
        let mut buf = [0; 1];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(buf[0])
    }
    fn read_i8_at(&self, pos: u64) -> Result<i8> {
        let mut buf = [0; 1];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(buf[0] as i8)
    }
    fn read_u16_at<T: ByteOrder>(&self, pos: u64) -> Result<u16> {
        let mut buf = [0; 2];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(T::read_u16(&buf))
    }
    fn read_i16_at<T: ByteOrder>(&self, pos: u64) -> Result<i16> {
        let mut buf = [0; 2];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(T::read_i16(&buf))
    }
    fn read_u32_at<T: ByteOrder>(&self, pos: u64) -> Result<u32> {
        let mut buf = [0; 4];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(T::read_u32(&buf))
    }
    fn read_i32_at<T: ByteOrder>(&self, pos: u64) -> Result<i32> {
        let mut buf = [0; 4];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(T::read_i32(&buf))
    }
    fn read_u64_at<T: ByteOrder>(&self, pos: u64) -> Result<u64> {
        let mut buf = [0; 8];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(T::read_u64(&buf))
    }
    fn read_i64_at<T: ByteOrder>(&self, pos: u64) -> Result<i64> {
        let mut buf = [0; 8];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(T::read_i64(&buf))
    }
    fn read_uint_at<T: ByteOrder>(&self, pos: u64, nbytes: usize) -> Result<u64> {
        let mut buf = [0; 8];
        try!(self.read_exact_at(&mut buf[..nbytes], pos));
        Ok(T::read_uint(&buf[..nbytes], nbytes))
    }
    fn read_int_at<T: ByteOrder>(&self, pos: u64, nbytes: usize) -> Result<i64> {
        let mut buf = [0; 8];
        try!(self.read_exact_at(&mut buf[..nbytes], pos));
        Ok(T::read_int(&buf[..nbytes], nbytes))
    }
    fn read_f32_at<T: ByteOrder>(&self, pos: u64) -> Result<f32> {
        let mut buf = [0; 4];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(T::read_f32(&buf))
    }
    fn read_f64_at<T: ByteOrder>(&self, pos: u64) -> Result<f64> {
        let mut buf = [0; 8];
        try!(self.read_exact_at(&mut buf, pos));
        Ok(T::read_f64(&buf))
    }
}
pub trait WriteBytesExt: WriteAt {
    fn write_u8_at(&mut self, pos: u64, n: u8) -> Result<()> {
        self.write_all_at(&[n], pos)
    }
    fn write_i8_at(&mut self, pos: u64, n: i8) -> Result<()> {
        self.write_all_at(&[n as u8], pos)
    }
    fn write_u16_at<T: ByteOrder>(&mut self, pos: u64, n: u16) -> Result<()> {
        let mut buf = [0; 2];
        T::write_u16(&mut buf, n);
        self.write_all_at(&buf, pos)
    }
    fn write_i16_at<T: ByteOrder>(&mut self, pos: u64, n: i16) -> Result<()> {
        let mut buf = [0; 2];
        T::write_i16(&mut buf, n);
        self.write_all_at(&buf, pos)
    }
    fn write_u32_at<T: ByteOrder>(&mut self, pos: u64, n: u32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, n);
        self.write_all_at(&buf, pos)
    }
    fn write_i32_at<T: ByteOrder>(&mut self, pos: u64, n: i32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_i32(&mut buf, n);
        self.write_all_at(&buf, pos)
    }
    fn write_u64_at<T: ByteOrder>(&mut self, pos: u64, n: u64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_u64(&mut buf, n);
        self.write_all_at(&buf, pos)
    }
    fn write_i64_at<T: ByteOrder>(&mut self, pos: u64, n: i64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_i64(&mut buf, n);
        self.write_all_at(&buf, pos)
    }
    fn write_uint_at<T: ByteOrder>(&mut self, pos: u64, n: u64, nbytes: usize) -> Result<()> {
        let mut buf = [0; 8];
        T::write_uint(&mut buf, n, nbytes);
        self.write_all_at(&buf[..nbytes], pos)
    }
    fn write_int_at<T: ByteOrder>(&mut self, pos: u64, n: i64, nbytes: usize) -> Result<()> {
        let mut buf = [0; 8];
        T::write_int(&mut buf, n, nbytes);
        self.write_all_at(&buf[..nbytes], pos)
    }
    fn write_f32_at<T: ByteOrder>(&mut self, pos: u64, n: f32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_f32(&mut buf, n);
        self.write_all_at(&buf, pos)
    }
    fn write_f64_at<T: ByteOrder>(&mut self, pos: u64, n: f64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_f64(&mut buf, n);
        self.write_all_at(&buf, pos)
    }
}


// Implement for everything that does positioned IO.
impl<R: ReadAt> ReadBytesExt for R {}
impl<W: WriteAt> WriteBytesExt for W {}

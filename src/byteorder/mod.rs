mod byteio;

use extbyteorder::ByteOrder;

use std::io::{Result, Read, Write};

use super::{ReadAt, WriteAt};

// Read/write bytes at positions.
trait ReadBytesExt {
    fn read_u8_at(&mut self) -> Result<u8> {
        unimplemented!()
    }
    fn read_i8_at(&mut self) -> Result<i8> {
        unimplemented!()
    }
    fn read_u16_at<T: ByteOrder>(&mut self) -> Result<u16> {
        unimplemented!()
    }
    fn read_i16_at<T: ByteOrder>(&mut self) -> Result<i16> {
        unimplemented!()
    }
    fn read_u32_at<T: ByteOrder>(&mut self) -> Result<u32> {
        unimplemented!()
    }
    fn read_i32_at<T: ByteOrder>(&mut self) -> Result<i32> {
        unimplemented!()
    }
    fn read_u64_at<T: ByteOrder>(&mut self) -> Result<u64> {
        unimplemented!()
    }
    fn read_i64_at<T: ByteOrder>(&mut self) -> Result<i64> {
        unimplemented!()
    }
    fn read_uint_at<T: ByteOrder>(&mut self, pos: u64, nbytes: usize) -> Result<u64> {
        unimplemented!()
    }
    fn read_int_at<T: ByteOrder>(&mut self, pos: u64, nbytes: usize) -> Result<i64> {
        unimplemented!()
    }
    fn read_f32_at<T: ByteOrder>(&mut self) -> Result<f32> {
        unimplemented!()
    }
    fn read_f64_at<T: ByteOrder>(&mut self) -> Result<f64> {
        unimplemented!()
    }
}
trait WriteBytesExt {
    fn write_u8_at(&mut self, pos: u64, n: u8) -> Result<()> {
        unimplemented!()
    }
    fn write_i8_at(&mut self, pos: u64, n: i8) -> Result<()> {
        unimplemented!()
    }
    fn write_u16_at<T: ByteOrder>(&mut self, pos: u64, n: u16) -> Result<()> {
        unimplemented!()
    }
    fn write_i16_at<T: ByteOrder>(&mut self, pos: u64, n: i16) -> Result<()> {
        unimplemented!()
    }
    fn write_u32_at<T: ByteOrder>(&mut self, pos: u64, n: u32) -> Result<()> {
        unimplemented!()
    }
    fn write_i32_at<T: ByteOrder>(&mut self, pos: u64, n: i32) -> Result<()> {
        unimplemented!()
    }
    fn write_u64_at<T: ByteOrder>(&mut self, pos: u64, n: u64) -> Result<()> {
        unimplemented!()
    }
    fn write_i64_at<T: ByteOrder>(&mut self, pos: u64, n: i64) -> Result<()> {
        unimplemented!()
    }
    fn write_uint_at<T: ByteOrder>(&mut self, pos: u64, n: u64, nbytes: usize) -> Result<()> {
        unimplemented!()
    }
    fn write_int_at<T: ByteOrder>(&mut self, pos: u64, n: i64, nbytes: usize) -> Result<()> {
        unimplemented!()
    }
    fn write_f32_at<T: ByteOrder>(&mut self, pos: u64, n: f32) -> Result<()> {
        unimplemented!()
    }
    fn write_f64_at<T: ByteOrder>(&mut self, pos: u64, n: f64) -> Result<()> {
        unimplemented!()
    }
}


// Implement for everything that does positioned IO.
impl<R: ReadAt> ReadBytesExt for R {}
impl<W: WriteAt> WriteBytesExt for W {}

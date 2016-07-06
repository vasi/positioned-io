use extbyteorder::ByteOrder;

use std::marker::PhantomData;
use std::io::{Result, Read, Write};

use super::super::{ReadAt, WriteAt};

// Read/write with a given endianness.
struct ByteIo<I, E: ByteOrder> {
    io: I,
    endianness: PhantomData<E>,
}
impl<I, E: ByteOrder> ByteIo<I, E>
    where I: Read
{
    fn read_u8(&mut self) -> Result<u8> {
        unimplemented!()
    }
    fn read_i8(&mut self) -> Result<i8> {
        unimplemented!()
    }
    fn read_u16(&mut self) -> Result<u16> {
        unimplemented!()
    }
    fn read_i16(&mut self) -> Result<i16> {
        unimplemented!()
    }
    fn read_u32(&mut self) -> Result<u32> {
        unimplemented!()
    }
    fn read_i32(&mut self) -> Result<i32> {
        unimplemented!()
    }
    fn read_u64(&mut self) -> Result<u64> {
        unimplemented!()
    }
    fn read_i64(&mut self) -> Result<i64> {
        unimplemented!()
    }
    fn read_uint(&mut self, pos: u64, nbytes: usize) -> Result<u64> {
        unimplemented!()
    }
    fn read_int(&mut self, pos: u64, nbytes: usize) -> Result<i64> {
        unimplemented!()
    }
    fn read_f32(&mut self) -> Result<f32> {
        unimplemented!()
    }
    fn read_f64(&mut self) -> Result<f64> {
        unimplemented!()
    }
}
impl<I, E: ByteOrder> ByteIo<I, E>
    where I: Write
{
    fn write_u8(&mut self, pos: u64, n: u8) -> Result<()> {
        unimplemented!()
    }
    fn write_i8(&mut self, pos: u64, n: i8) -> Result<()> {
        unimplemented!()
    }
    fn write_u16(&mut self, pos: u64, n: u16) -> Result<()> {
        unimplemented!()
    }
    fn write_i16(&mut self, pos: u64, n: i16) -> Result<()> {
        unimplemented!()
    }
    fn write_u32(&mut self, pos: u64, n: u32) -> Result<()> {
        unimplemented!()
    }
    fn write_i32(&mut self, pos: u64, n: i32) -> Result<()> {
        unimplemented!()
    }
    fn write_u64(&mut self, pos: u64, n: u64) -> Result<()> {
        unimplemented!()
    }
    fn write_i64(&mut self, pos: u64, n: i64) -> Result<()> {
        unimplemented!()
    }
    fn write_uint(&mut self, pos: u64, n: u64, nbytes: usize) -> Result<()> {
        unimplemented!()
    }
    fn write_int(&mut self, pos: u64, n: i64, nbytes: usize) -> Result<()> {
        unimplemented!()
    }
    fn write_f32(&mut self, pos: u64, n: f32) -> Result<()> {
        unimplemented!()
    }
    fn write_f64(&mut self, pos: u64, n: f64) -> Result<()> {
        unimplemented!()
    }
}
impl<I, E: ByteOrder> ByteIo<I, E>
    where I: ReadAt
{
    fn read_u8_at(&mut self) -> Result<u8> {
        unimplemented!()
    }
    fn read_i8_at(&mut self) -> Result<i8> {
        unimplemented!()
    }
    fn read_u16_at(&mut self) -> Result<u16> {
        unimplemented!()
    }
    fn read_i16_at(&mut self) -> Result<i16> {
        unimplemented!()
    }
    fn read_u32_at(&mut self) -> Result<u32> {
        unimplemented!()
    }
    fn read_i32_at(&mut self) -> Result<i32> {
        unimplemented!()
    }
    fn read_u64_at(&mut self) -> Result<u64> {
        unimplemented!()
    }
    fn read_i64_at(&mut self) -> Result<i64> {
        unimplemented!()
    }
    fn read_uint_at(&mut self, pos: u64, nbytes: usize) -> Result<u64> {
        unimplemented!()
    }
    fn read_int_at(&mut self, pos: u64, nbytes: usize) -> Result<i64> {
        unimplemented!()
    }
    fn read_f32_at(&mut self) -> Result<f32> {
        unimplemented!()
    }
    fn read_f64_at(&mut self) -> Result<f64> {
        unimplemented!()
    }
}
impl<I, E: ByteOrder> ByteIo<I, E>
    where I: WriteAt
{
    fn write_u8_at(&mut self, pos: u64, n: u8) -> Result<()> {
        unimplemented!()
    }
    fn write_i8_at(&mut self, pos: u64, n: i8) -> Result<()> {
        unimplemented!()
    }
    fn write_u16_at(&mut self, pos: u64, n: u16) -> Result<()> {
        unimplemented!()
    }
    fn write_i16_at(&mut self, pos: u64, n: i16) -> Result<()> {
        unimplemented!()
    }
    fn write_u32_at(&mut self, pos: u64, n: u32) -> Result<()> {
        unimplemented!()
    }
    fn write_i32_at(&mut self, pos: u64, n: i32) -> Result<()> {
        unimplemented!()
    }
    fn write_u64_at(&mut self, pos: u64, n: u64) -> Result<()> {
        unimplemented!()
    }
    fn write_i64_at(&mut self, pos: u64, n: i64) -> Result<()> {
        unimplemented!()
    }
    fn write_uint_at(&mut self, pos: u64, n: u64, nbytes: usize) -> Result<()> {
        unimplemented!()
    }
    fn write_int_at(&mut self, pos: u64, n: i64, nbytes: usize) -> Result<()> {
        unimplemented!()
    }
    fn write_f32_at(&mut self, pos: u64, n: f32) -> Result<()> {
        unimplemented!()
    }
    fn write_f64_at(&mut self, pos: u64, n: f64) -> Result<()> {
        unimplemented!()
    }
}

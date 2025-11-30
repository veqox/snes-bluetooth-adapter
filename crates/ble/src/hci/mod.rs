use core::{convert::Infallible, fmt::Debug, u16, usize};

use embedded_io::{ErrorType, Write};

pub mod command;
pub mod event;
pub mod gap;
pub mod packet;

pub struct MacAddr(pub [u8; 6]);

impl Debug for MacAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let b = self.0;
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            b[0], b[1], b[2], b[3], b[4], b[5]
        )
    }
}

pub(crate) struct BufWriter<'a> {
    pub pos: usize,
    pub buf: &'a mut [u8],
}

impl<'a> BufWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { pos: 0, buf }
    }
}

impl ErrorType for BufWriter<'_> {
    type Error = Infallible; // can technically fail but whatever
}

impl Write for BufWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Self::Error> {
        let end = self.pos + bytes.len();
        self.buf[self.pos..end].copy_from_slice(bytes);
        self.pos = end;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait WriteHCI {
    fn write_into<W: Write>(&self, w: &mut W) -> Result<(), W::Error>;
}

impl WriteHCI for u8 {
    fn write_into<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        w.write_all(&[*self])
    }
}

impl WriteHCI for bool {
    fn write_into<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        w.write_all(&[(*self).into()])
    }
}

impl WriteHCI for u16 {
    fn write_into<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        w.write_all(&self.to_le_bytes())
    }
}

impl WriteHCI for [u8] {
    fn write_into<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        w.write_all(self)
    }
}

pub(crate) struct BufReader<'a> {
    pub pos: usize,
    pub buf: &'a [u8],
}

impl<'a> BufReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { pos: 0, buf }
    }

    pub fn read_u8(&mut self) -> u8 {
        let b = self.buf[self.pos];
        self.pos += 1;
        b
    }

    pub fn read_u16_le(&mut self) -> u16 {
        u16::from_le_bytes([self.read_u8(), self.read_u8()])
    }

    pub fn read_bytes(&mut self, len: usize) -> &'a [u8] {
        let bytes = &self.buf[self.pos..self.pos + len];
        self.pos += len;
        bytes
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }
}

pub(crate) trait ReadHCI<'a> {
    fn read_from(r: &mut BufReader<'a>) -> Self;
}

impl<'a> ReadHCI<'a> for u8 {
    fn read_from(r: &mut BufReader) -> Self {
        r.read_u8()
    }
}

impl<'a> ReadHCI<'a> for u16 {
    fn read_from(r: &mut BufReader) -> Self {
        r.read_u16_le()
    }
}

impl<'a> ReadHCI<'a> for MacAddr {
    fn read_from(r: &mut BufReader) -> Self {
        MacAddr(r.read_bytes(6).try_into().unwrap())
    }
}

impl<'a> ReadHCI<'a> for &'a [u8] {
    fn read_from(r: &mut BufReader<'a>) -> Self {
        r.read_bytes(r.remaining())
    }
}

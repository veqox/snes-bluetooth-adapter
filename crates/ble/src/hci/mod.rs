use core::{convert::Infallible, usize};

use embedded_io::{ErrorType, Write};

pub mod command;
pub mod gap;

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

#![no_std]

#[derive(Debug)]
pub enum WriteError {
    BufferOverflow,
}

#[derive(Debug)]
pub struct Writer<'p> {
    buf: &'p mut [u8],
    pub pos: usize,
}

impl<'p> Writer<'p> {
    pub fn new(buf: &'p mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), WriteError> {
        self.write_slice(&value.to_le_bytes())
    }

    pub fn write_u16(&mut self, value: u16) -> Result<(), WriteError> {
        self.write_slice(&value.to_le_bytes())
    }

    pub fn write_u32(&mut self, value: u32) -> Result<(), WriteError> {
        self.write_slice(&value.to_le_bytes())
    }

    pub fn write_u64(&mut self, value: u64) -> Result<(), WriteError> {
        self.write_slice(&value.to_le_bytes())
    }

    pub fn write_u128(&mut self, value: u128) -> Result<(), WriteError> {
        self.write_slice(&value.to_le_bytes())
    }

    pub fn write_slice(&mut self, slice: &[u8]) -> Result<(), WriteError> {
        if self.pos + slice.len() >= self.buf.len() {
            return Err(WriteError::BufferOverflow);
        }

        self.buf[self.pos..(self.pos + slice.len())].copy_from_slice(slice);
        self.pos += slice.len();

        Ok(())
    }
}

#[derive(Debug)]
pub struct Reader<'p> {
    buf: &'p [u8],
    pub pos: usize,
}

impl<'p> Reader<'p> {
    pub fn new(buf: &'p [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        Some(u8::from_le_bytes(
            self.read_slice(size_of::<u8>())?.try_into().ok()?,
        ))
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(
            self.read_slice(size_of::<u16>())?.try_into().ok()?,
        ))
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(
            self.read_slice(size_of::<u32>())?.try_into().ok()?,
        ))
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(
            self.read_slice(size_of::<u64>())?.try_into().ok()?,
        ))
    }

    pub fn read_u128(&mut self) -> Option<u128> {
        Some(u128::from_le_bytes(
            self.read_slice(size_of::<u128>())?.try_into().ok()?,
        ))
    }

    pub fn read_slice(&mut self, len: usize) -> Option<&'p [u8]> {
        if self.remaining() < len {
            return None;
        }

        let slice = &self.buf[self.pos..(self.pos + len)];
        self.pos += len;
        Some(slice)
    }

    pub fn seek(&mut self, pos: usize) {
        if pos > self.buf.len() {
            // todo: should probably do some kind of error handling
            return;
        }

        self.pos = pos
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }
}

pub trait SliceAs<'p> {
    unsafe fn as_u8_slice(&self) -> Option<&'p [u8]>;
    unsafe fn as_u16_slice(&self) -> Option<&'p [u16]>;
    unsafe fn as_u32_slice(&self) -> Option<&'p [u32]>;
    unsafe fn as_u64_slice(&self) -> Option<&'p [u64]>;
    unsafe fn as_u128_slice(&self) -> Option<&'p [u128]>;
}

impl<'p, T> SliceAs<'p> for &'p [T] {
    unsafe fn as_u8_slice(&self) -> Option<&'p [u8]> {
        as_slice(self)
    }

    unsafe fn as_u16_slice(&self) -> Option<&'p [u16]> {
        as_slice(self)
    }

    unsafe fn as_u32_slice(&self) -> Option<&'p [u32]> {
        as_slice(self)
    }

    unsafe fn as_u64_slice(&self) -> Option<&'p [u64]> {
        as_slice(self)
    }

    unsafe fn as_u128_slice(&self) -> Option<&'p [u128]> {
        as_slice(self)
    }
}

const unsafe fn as_slice<T, U>(slice: &[U]) -> Option<&[T]> {
    if slice.len() % size_of::<T>() != 0 {
        return None;
    }

    Some(core::slice::from_raw_parts(
        slice.as_ptr() as *const T,
        slice.len() / size_of::<T>(),
    ))
}

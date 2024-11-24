#![no_std]

pub struct Writer<'p> {
    buf: &'p mut [u8],
    pub pos: usize,
}

impl<'p> Writer<'p> {
    pub fn new(buf: &'p mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buf[self.pos] = value;
        self.pos += 1;
    }

    pub fn write_u16(&mut self, value: u16) {
        self.buf[self.pos..(self.pos + 2)].copy_from_slice(&value.to_le_bytes());
        self.pos += 2;
    }

    pub fn write_slice(&mut self, slice: &[u8]) {
        self.buf[self.pos..(self.pos + slice.len())].copy_from_slice(slice);
        self.pos += slice.len();
    }
}

pub struct Reader<'p> {
    buf: &'p [u8],
    pub pos: usize,
}

impl<'p> Reader<'p> {
    pub fn new(buf: &'p [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn read_u8(&mut self) -> u8 {
        let value = self.buf[self.pos];
        self.pos += 1;
        value
    }

    pub fn read_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes([self.buf[self.pos], self.buf[self.pos + 1]]);
        self.pos += 2;
        value
    }

    pub fn read_slice(&mut self, len: usize) -> &'p [u8] {
        let slice = &self.buf[self.pos..(self.pos + len)];
        self.pos += len;
        slice
    }
}

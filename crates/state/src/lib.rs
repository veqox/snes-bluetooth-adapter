#![no_std]

/*
Bits Offset Length Name        Values
0    0      1      B           0-1
0    1      1      Y           0-1
0    2      1      Select      0-1
0    3      1      Start       0-1
0    4      1      Up          0-1
0    5      1      Down        0-1
0    6      1      Left        0-1
0    7      1      Right       0-1
0    8      1      A           0-1
0    9      1      X           0-1
0    10     1      L           0-1
0    11     1      R           0-1
0000 12     4      Clock Cycle 0-15

0 = Pressed (false/low)
1 = Not Pressed (true/high)
*/

use core::fmt::Display;

const B_OFFSET: u16 = 0;
const Y_OFFSET: u16 = 1;
const SELECT_OFFSET: u16 = 2;
const START_OFFSET: u16 = 3;
const UP_OFFSET: u16 = 4;
const DOWN_OFFSET: u16 = 5;
const LEFT_OFFSET: u16 = 6;
const RIGHT_OFFSET: u16 = 7;
const A_OFFSET: u16 = 8;
const X_OFFSET: u16 = 9;
const L_OFFSET: u16 = 10;
const R_OFFSET: u16 = 11;
const CYCLE_OFFSET: u16 = 12;

const CYCLE_LEN: u16 = 4;

pub struct State {
    value: u16,
}

impl Default for State {
    fn default() -> Self {
        Self {
            value: 0b0000_1111_1111_1111,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:16b}", self.value)
    }
}

impl State {
    pub fn next(&mut self) -> bool {
        let cycle = self.cycle();
        self.set_cycle(cycle + 1);

        match cycle {
            x if x < 12 => self.read_bit(cycle),
            _ => true,
        }
    }

    pub fn cycle(&self) -> u16 {
        self.read(CYCLE_LEN, CYCLE_OFFSET)
    }

    fn read_bit(&self, offset: u16) -> bool {
        self.read(1, offset) == 1
    }

    fn read(&self, len: u16, offset: u16) -> u16 {
        const N: u16 = u16::BITS as u16;
        self.value >> offset << (N - len) >> (N - len)
    }

    pub fn set_b(&mut self, value: bool) {
        self.write_bit(value.into(), B_OFFSET);
    }

    pub fn set_y(&mut self, value: bool) {
        self.write_bit(value.into(), Y_OFFSET);
    }

    pub fn set_select(&mut self, value: bool) {
        self.write_bit(value.into(), SELECT_OFFSET);
    }

    pub fn set_start(&mut self, value: bool) {
        self.write_bit(value.into(), START_OFFSET);
    }

    pub fn set_up(&mut self, value: bool) {
        self.write_bit(value.into(), UP_OFFSET);
    }

    pub fn set_down(&mut self, value: bool) {
        self.write_bit(value.into(), DOWN_OFFSET);
    }

    pub fn set_left(&mut self, value: bool) {
        self.write_bit(value.into(), LEFT_OFFSET);
    }

    pub fn set_right(&mut self, value: bool) {
        self.write_bit(value.into(), RIGHT_OFFSET);
    }

    pub fn set_a(&mut self, value: bool) {
        self.write_bit(value.into(), A_OFFSET);
    }

    pub fn set_x(&mut self, value: bool) {
        self.write_bit(value.into(), X_OFFSET);
    }

    pub fn set_l(&mut self, value: bool) {
        self.write_bit(value.into(), L_OFFSET);
    }

    pub fn set_r(&mut self, value: bool) {
        self.write_bit(value.into(), R_OFFSET);
    }

    pub fn reset_cycle(&mut self) {
        self.set_cycle(0);
    }

    /// Sets the cycle to the given value.
    /// When the value exceeds 15 it will be automatically cut off because the cycle bits are located at the end of the value
    /// When writing the value it is shifted by 12 bits to the left cutting off the fifth bit making it impossible to exceed 15
    fn set_cycle(&mut self, value: u16) {
        self.write(value, CYCLE_LEN, CYCLE_OFFSET);
    }

    fn write_bit(&mut self, value: bool, offset: u16) {
        self.write(value.into(), 1, offset);
    }

    fn write(&mut self, value: u16, len: u16, offset: u16) {
        let mask = u16::MAX >> (u16::BITS as u16 - len);
        let value = (value as u16) & mask;
        self.value &= !(mask << offset);
        self.value |= value << offset;
    }
}

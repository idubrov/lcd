#[macro_use]
extern crate pretty_assertions;
extern crate lcd;

mod util;

use lcd::{FunctionMode, FunctionLine, FunctionDots};

// This one makes BufferHardware to be "busy-capable"
pub struct InputData {
    data: Vec<u8>,
}

impl<'a> lcd::InputCapableHardware for util::BufferHardware<'a, InputData> {
    fn rw(&self, bit: bool) {
        self.command(format!("RW {}", bit));
    }

    fn read_data(&self) -> u8 {
        self.command(format!("IS BUSY?"));
        self.busy.borrow_mut().data.remove(0)
    }
}

fn with_data(data: Vec<u8>) -> InputData {
    InputData {
        data
    }
}

#[test]
fn init_4bit() {
    let vec = util::test(FunctionMode::Bit4, with_data(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), |lcd| {
        lcd.init(FunctionLine::Line2, FunctionDots::Dots5x8);
    });
    assert_eq!(vec, vec![
        // Send init command three times
        "R/S false",
        "DATA 0b0011",
        "EN true", "DELAY 1", "EN false", "DELAY 4500",
        "EN true", "DELAY 1", "EN false", "DELAY 150",
        "EN true", "DELAY 1", "EN false",
        // Check busy
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        // Switch to 4-bit mode
        "DATA 0b0010", "EN true", "DELAY 1", "EN false",
        // Check busy
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        // Set lines, font size
        "R/S false",
        "DATA 0b0010", "EN true", "DELAY 1", "EN false",
        "DATA 0b1000", "EN true", "DELAY 1", "EN false",
        // Check busy
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        // Display
        "R/S false",
        "DATA 0b0000", "EN true", "DELAY 1", "EN false",
        "DATA 0b1000", "EN true", "DELAY 1", "EN false",
        // Check busy
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        // Clear
        "R/S false",
        "DATA 0b0000", "EN true", "DELAY 1", "EN false",
        "DATA 0b0001", "EN true", "DELAY 1", "EN false",
        // Check busy
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        // FIXME: no harm, but should not really wait for busy again...
        // Check busy again
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        // Entry mode
        "R/S false",
        "DATA 0b0000", "EN true", "DELAY 1", "EN false",
        "DATA 0b0110", "EN true", "DELAY 1", "EN false",
        // Check busy
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false"
    ]);
}

#[test]
fn write_4bit() {
    let vec = util::test(FunctionMode::Bit4, with_data(vec![0, 0]), |lcd| {
        lcd.write('a' as u8);
    });
    assert_eq!(vec, vec![
        "R/S true",
        "DATA 0b0110", "EN true", "DELAY 1", "EN false",
        "DATA 0b0001", "EN true", "DELAY 1", "EN false",
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        "DELAY 5"
    ]);
}

#[test]
fn write_8bit() {
    let vec = util::test(FunctionMode::Bit8, with_data(vec![0]), |lcd| {
        lcd.write('a' as u8);
    });
    assert_eq!(vec, vec![
        "R/S true",
        "DATA 0b01100001", "EN true", "DELAY 1", "EN false",
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        "DELAY 5"
    ]);
}

#[test]
fn write_4bit_long_busy() {
    let vec = util::test(FunctionMode::Bit4, with_data(vec![8, 0, 8, 0, 8, 0, 0, 0]), |lcd| {
        lcd.write('a' as u8);
    });
    assert_eq!(vec, vec![
        "R/S true",
        "DATA 0b0110", "EN true", "DELAY 1", "EN false",
        "DATA 0b0001", "EN true", "DELAY 1", "EN false",
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        "DELAY 5"
    ]);
}

#[test]
fn write_8bit_long_busy() {
    let vec = util::test(FunctionMode::Bit8, with_data(vec![128, 128, 128, 0]), |lcd| {
        lcd.write('a' as u8);
    });
    assert_eq!(vec, vec![
        "R/S true",
        "DATA 0b01100001", "EN true", "DELAY 1", "EN false",
        "R/S false",
        "RW true",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "IS BUSY?", "DELAY 1", "EN false",
        "RW false",
        "DELAY 5"
    ]);
}
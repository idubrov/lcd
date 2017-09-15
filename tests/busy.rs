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
        self.command(format!("BUSY..."));
        self.busy.borrow_mut().data.pop().unwrap()
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
        "RW true", "R/S false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "RW false",
        // Switch to 4-bit mode
        "DATA 0b0010", "EN true", "DELAY 1", "EN false",
        // Check busy
        "RW true", "R/S false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "RW false",
        // Set lines, font size
        "R/S false",
        "DATA 0b0010", "EN true", "DELAY 1", "EN false",
        "DATA 0b1000", "EN true", "DELAY 1", "EN false",
        // Check busy
        "RW true", "R/S false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "RW false",
        // Display
        "R/S false",
        "DATA 0b0000", "EN true", "DELAY 1", "EN false",
        "DATA 0b1000", "EN true", "DELAY 1", "EN false",
        // Check busy
        "RW true", "R/S false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "RW false",
        // Clear
        "R/S false",
        "DATA 0b0000", "EN true", "DELAY 1", "EN false",
        "DATA 0b0001", "EN true", "DELAY 1", "EN false",
        // Check busy
        "RW true", "R/S false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "RW false",
        // FIXME: no harm, but should not really wait for busy again...
        // Check busy again
        "RW true", "R/S false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "RW false",
        // Entry mode
        "R/S false",
        "DATA 0b0000", "EN true", "DELAY 1", "EN false",
        "DATA 0b0110", "EN true", "DELAY 1", "EN false",
        // Check busy
        "RW true", "R/S false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "EN true", "DELAY 1", "BUSY...", "DELAY 1", "EN false",
        "RW false"
    ]);
}
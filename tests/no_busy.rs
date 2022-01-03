#[macro_use]
extern crate pretty_assertions;
extern crate lcd;

mod util;
use lcd::{
    Direction, EntryModeDirection, EntryModeShift, FunctionDots, FunctionLine, FunctionMode,
};

#[test]
fn init_4bit() {
    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.init(FunctionLine::Line2, FunctionDots::Dots5x8);
    });
    assert_eq!(
        vec,
        vec![
            // Send init command three times
            "R/S false",
            "DATA 0b0011",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 4500",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 150",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            // Switch to 4-bit mode
            "DATA 0b0010",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            // Set lines, font size
            "R/S false",
            "DATA 0b0010",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            // Display
            "R/S false",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            // Clear
            "R/S false",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 2000",
            // Entry mode
            "R/S false",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0110",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );
}

#[test]
fn init_8bit() {
    let vec = util::test(FunctionMode::Bit8, None, |lcd| {
        lcd.init(FunctionLine::Line2, FunctionDots::Dots5x8);
    });
    assert_eq!(
        vec,
        vec![
            // Send init command three times
            "R/S false",
            "DATA 0b00111100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 4500",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 150",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            // Set lines, font size
            "R/S false",
            "DATA 0b00111000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            // Display
            "R/S false",
            "DATA 0b00001000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            // Clear
            "R/S false",
            "DATA 0b00000001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 2000",
            // Entry mode
            "R/S false",
            "DATA 0b00000110",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );
}

#[test]
fn clear_4bit() {
    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.clear();
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 2000"
        ]
    );
}

#[test]
fn clear_8bit() {
    let vec = util::test(FunctionMode::Bit8, None, |lcd| {
        lcd.clear();
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b00000001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 2000"
        ]
    );
}

#[test]
fn home_4bit() {
    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.home();
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0010",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 2000"
        ]
    );
}

#[test]
fn home_8bit() {
    let vec = util::test(FunctionMode::Bit8, None, |lcd| {
        lcd.home();
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b00000010",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 2000"
        ]
    );
}

#[test]
fn entry_mode_4bit() {
    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.entry_mode(EntryModeDirection::EntryLeft, EntryModeShift::NoShift);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );

    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::Shift);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0111",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );
}

#[test]
fn scroll_4bit() {
    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.scroll(Direction::Left);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b0001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );

    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.scroll(Direction::Right);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b0001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );
}

#[test]
fn cursor_4bit() {
    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.cursor(Direction::Left);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b0001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );

    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.cursor(Direction::Right);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b0001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );
}

#[test]
fn position_4bit() {
    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.position(3, 0);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b1000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0011",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );

    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.position(3, 1);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b1100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0011",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );

    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.position(7, 2);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b1001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1011",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );

    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.position(8, 3);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b1101",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50"
        ]
    );
}

#[test]
fn print() {
    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.print("hello");
    });
    assert_eq!(
        vec,
        vec![
            "R/S true",
            "DATA 0b0110",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0110",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0101",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0110",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0110",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0110",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1111",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5"
        ]
    );
}

#[test]
fn upload() {
    const ARROW: [u8; 8] = [
        0b00000, 0b01000, 0b01100, 0b01110, 0b11111, 0b01110, 0b01100, 0b01000,
    ];

    let vec = util::test(FunctionMode::Bit4, None, |lcd| {
        lcd.upload_character(3, ARROW);
    });
    assert_eq!(
        vec,
        vec![
            "R/S false",
            "DATA 0b0101",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "R/S true",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1110",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0001",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1111",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1110",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1100",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
            "R/S true",
            "DATA 0b0000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DATA 0b1000",
            "EN true",
            "DELAY 1",
            "EN false",
            "DELAY 50",
            "DELAY 5",
        ]
    );
}

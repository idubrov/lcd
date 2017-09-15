#![no_std]
#![feature(const_fn)]
#![deny(warnings)]

//! Library that implements low-level protocol to the [Hitachi HD44780][1]-compatible LCD device.
//!
//! Provides high-level API to the [Hitachi HD44780][1]-compatible LCD device. Uses 4-bit mode
//! by default (only uses 4 data pins) plus two control pins (R/S and EN). R/W pin is not used
//! and should be wired for "write" (low-level, 0).
//!
//! The implementation is completely stateless. Client is free to reuse the same `Display` object
//! or to create one every time access to LCD is required.
//!
//! `Display` also implements `core::fmt::Write` trait, so it could be used as a target of `write!`
//! macro.
//!
//! This library does not depend on `std` crate and could be used in bare metal embedded development.
//!
//! # Examples
//! ```rust,no_run
//! #![no_std]
//! # #![feature(lang_items)]
//! # #![feature(start)]
//! # #[lang = "eh_personality"] fn eh_personality() {}
//! # #[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
//! # #[start]
//! # fn start(_argc: isize, _argv: *const *const u8) -> isize { 0 }
//! use core::fmt::Write; // for write!
//! use lcd::*;
//!
//! // implement HAL...
//! struct HW {
//!     // any data needed to access low-level peripherals
//! }
//!
//! // implement `Hardware` trait to give access to LCD pins
//! impl Hardware for HW {
//!     fn rs(&self, bit: bool) {
//!         // should set R/S pin on LCD screen
//!     }
//!     fn enable(&self, bit: bool) {
//!         // should set EN pin on LCD screen
//!     }
//!     fn data(&self, data: u8) {
//!         // should set data bits to the LCD screen (only lowest 4 bits are used in 4-bit mode).
//!     }
//!
//!     // optionally, override the following function to switch to 8-bit mode
//!     fn mode(&self) -> lcd::FunctionMode {
//!         lcd::FunctionMode::Bit8
//!     }
//! }
//!
//! // implement `Delay` trait to allow library to sleep for given amount of time
//! impl Delay for HW {
//!     fn delay_us(&self, delay_usec: u32) {
//!         // should delay for the given amount of microseconds
//!     }
//! }
//!
//! // create HAL and LCD instances
//! let hw = HW { /* ... */ };
//! let mut lcd = Display::new(hw);
//!
//! // initialization
//! lcd.init(FunctionLine::Line2, FunctionDots::Dots5x8);
//! lcd.display(
//!     DisplayMode::DisplayOn,
//!     DisplayCursor::CursorOff,
//!     DisplayBlink::BlinkOff);
//! lcd.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::NoShift);
//!
//! // print something
//! write!(&mut lcd, "Hello, my number today is {: >4}", 42).unwrap();
//! ```
//!
//! [1]: https://en.wikipedia.org/wiki/Hitachi_HD44780_LCD_controller


#[cfg(feature = "fast-format")]
extern crate fast_fmt;

#[derive(Copy, Clone, Debug)]
pub enum FunctionMode {
    /// Send data 4 bits at the time
    Bit4 = 0x00,
    /// Send data 8 bits at the time
    Bit8 = 0x10
}

#[derive(Copy, Clone, Debug)]
pub enum FunctionDots {
    Dots5x8 = 0x00,
    Dots5x10 = 0x04
}

#[derive(Copy, Clone, Debug)]
pub enum FunctionLine {
    Line1 = 0x00,
    Line2 = 0x08
}

#[derive(Copy, Clone, Debug)]
pub enum DisplayBlink {
    BlinkOff = 0x00,
    BlinkOn = 0x01
}

#[derive(Copy, Clone, Debug)]
pub enum DisplayCursor {
    CursorOff = 0x00,
    CursorOn = 0x02
}

#[derive(Copy, Clone, Debug)]
pub enum DisplayMode {
    DisplayOff = 0x00,
    DisplayOn = 0x04
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Left = 0x00,
    Right = 0x04
}

#[derive(Copy, Clone, Debug)]
pub enum Scroll {
    CursorMove = 0x00,
    DisplayMove = 0x08
}

#[derive(Copy, Clone, Debug)]
pub enum EntryModeDirection {
    EntryLeft = 0x00,
    EntryRight = 0x02
}

#[derive(Copy, Clone, Debug)]
pub enum EntryModeShift {
    NoShift = 0x00,
    Shift = 0x01
}

#[derive(Copy, Clone, Debug)]
pub enum Command {
    ClearDisplay = 0x01,
    ReturnHome = 0x02,
    EntryModeSet = 0x04,
    DisplayControl = 0x08,
    CursorShift = 0x10,
    FunctionSet = 0x20,
    SetCGRamAddr = 0x40,
    SetDDRamAddr = 0x80
}

pub trait Delay {
    /// Delay for given amount of time (in microseconds).
    fn delay_us(&self, delay_usec: u32);
}

pub trait Hardware {
    fn rs(&self, bit: bool);
    fn enable(&self, bit: bool);
    fn data(&self, data: u8);

    /// Address set up time is 40ns minimum (tAS)
    /// This function should be overridden in case processor is too fast for 40ns to pass.
    fn wait_address(&self) {}

    /// Override to pick 8-bit mode (4-bit mode by default)
    fn mode(&self) -> FunctionMode {
        FunctionMode::Bit4
    }
}

/// Object implementing HD44780 protocol. Stateless (could be created as many times as needed).
pub struct Display<HW: Hardware> {
    hw: HW
}

impl<HW: Hardware + Delay> core::fmt::Write for Display<HW> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

#[cfg(feature = "fast-format")]
impl<HW: Hardware + Delay> fast_fmt::Write for Display<HW> {
    type Error = ();

    fn write_char(&mut self, val: char) -> Result<(), Self::Error> {
        self.write(val as u8);
        Ok(())
    }

    fn size_hint(&mut self, _bytes: usize) {}
}

impl<HW: Hardware + Delay> Display<HW> {
    pub fn new(hw: HW) -> Display<HW> {
        Display {
            hw: hw
        }
    }

    /// Initialize LCD display. Sets an equivalent of the following setup:
    ///
    /// ```rust,no_run
    /// # use lcd::*;
    /// # struct HW {}
    /// # impl Hardware for HW {
    /// #   fn rs(&self, bit: bool) { }
    /// #   fn enable(&self, bit: bool) { }
    /// #   fn data(&self, data: u8) { }
    /// # }
    /// # impl Delay for HW {
    /// #   fn delay_us(&self, delay_usec: u32) { }
    /// # }
    /// # let mut lcd = Display::new(HW {});
    /// lcd.display(DisplayMode::DisplayOff, DisplayCursor::CursorOff, DisplayBlink::BlinkOff);
    /// lcd.clear();
    /// lcd.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::NoShift);
    /// ```
    pub fn init(&mut self, line: FunctionLine, dots: FunctionDots) {
        let mode = self.hw.mode();
        self.hw.rs(false);
        self.hw.wait_address();
        match mode {
            FunctionMode::Bit8 => {
                // Run initialization procedure for the display (8-bit mode).

                // Set to 8-bit mode, 2 line, 5x10 font
                // Display off, clear, entry mode set
                self.send_data((Command::FunctionSet as u8) |
                    (FunctionMode::Bit8 as u8) |
                    (FunctionLine::Line2 as u8) |
                    (FunctionDots::Dots5x10 as u8)); // Send command for the first time

                self.hw.delay_us(4500); // Wait for more than 4.1ms

                self.pulse_enable(); // Repeat for the second time
                self.hw.delay_us(150); // Wait for more than 100us

                self.pulse_enable(); // Repeat for the third time
                self.wait_ready_default();
            }
            FunctionMode::Bit4 => {
                // Run initialization procedure for the display (4-bit mode).
                self.send_data(((Command::FunctionSet as u8) | (FunctionMode::Bit8 as u8)) >> 4);
                self.hw.delay_us(4500); // Wait for more than 4.1ms

                self.pulse_enable(); // Repeat for the second time
                self.hw.delay_us(150); // Wait for more than 100us

                self.pulse_enable(); // Repeat for the third time
                self.wait_ready_default(); // Wait fo FunctionSet to finish

                // Now we switch to 4-bit mode
                self.send_data(((Command::FunctionSet as u8) | (FunctionMode::Bit4 as u8)) >> 4);
                self.wait_ready_default(); // Wait for FunctionSet to finish
            }
        }

        // Finally, set # lines, font size
        self.command((Command::FunctionSet as u8) | (mode as u8) | (line as u8) | (dots as u8));

        // Now display should be properly initialized, we can check BF now
        // Though if we are not checking BF, waiting time is longer
        self.display(DisplayMode::DisplayOff, DisplayCursor::CursorOff, DisplayBlink::BlinkOff);
        self.clear();
        self.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::NoShift);
    }


    /// Clears display and returns cursor to the home position (address 0).
    pub fn clear(&mut self) -> &Self {
        self.command(Command::ClearDisplay as u8);
        // This command could take as long as 1.52ms to execute
        self.wait_ready(2000);
        self
    }

    /// Returns cursor to home position. Also returns display being shifted to the original position.
    /// DDRAM content remains unchanged.
    pub fn home(&mut self) -> &Self {
        self.command(Command::ReturnHome as u8);
        // This command could take as long as 1.52ms to execute
        self.wait_ready(2000);
        self
    }

    /// Sets cursor move direction (`entry`); specifies to shift the display (`scroll`).
    /// These operations are performed during data read/write.
    pub fn entry_mode(&mut self, dir: EntryModeDirection, scroll: EntryModeShift) -> &Self {
        self.command((Command::EntryModeSet as u8) | (dir as u8) | (scroll as u8))
    }

    /// Sets on/off of all display (`display`), cursor on/off (`cursor`), and blink of cursor
    /// position character (`blink`).
    pub fn display(&mut self, display: DisplayMode, cursor: DisplayCursor, blink: DisplayBlink) -> &Self {
        self.command((Command::DisplayControl as u8) | (display as u8) | (cursor as u8) | (blink as u8))
    }

    /// Sets display-shift, direction (`dir`). DDRAM content remains unchanged.
    pub fn scroll(&mut self, dir: Direction) -> &Self {
        self.command((Command::CursorShift as u8) | (Scroll::DisplayMove as u8) | (dir as u8))
    }

    /// Sets cursor-shift, direction (`dir`). DDRAM content remains unchanged.
    pub fn cursor(&mut self, dir: Direction) -> &Self {
        self.command((Command::CursorShift as u8) | (Scroll::CursorMove as u8) | (dir as u8))
    }

    /// Sets the cursor position to the given row (`row`) and column (`col`).
    pub fn position(&mut self, col: u8, row: u8) {
        let offset = match row {
            1 => 0x40,
            2 => 0x14,
            3 => 0x54,
            _ => 0
        };
        self.command((Command::SetDDRamAddr as u8) | (col + offset));
    }

    /// Print given string (`str`) on the LCD screen.
    pub fn print(&mut self, str: &str) -> &Self {
        for c in str.as_bytes() {
            self.write(*c);
        }
        self
    }

    /// Write given character (given as `data` of type `u8`) on the LCD screen.
    pub fn write(&mut self, data: u8) -> &Self {
        self.hw.rs(true);
        self.hw.wait_address(); // tAS
        self.send(data);
        self.wait_ready_default();
        // It takes 4us more (tADD) to update address counter
        self.hw.delay_us(5);
        self
    }

    /// Upload character image at given location. Only locations 0-7 are supported (panics otherwise).
    /// Each character is represented by an array of 8 bytes, each byte being a row.
    /// Only 5 bits are used from each byte (representing columns).
    pub fn upload_character(&mut self, location: u8, map: [u8; 8]) -> &Self {
        assert!(location <= 7);

        // Only 8 locations are available
        self.command((Command::SetCGRamAddr as u8) | ((location & 0x7) << 3));
        for item in map.iter().take(8) {
            self.write(*item);
        }
        self
    }

    fn command(&mut self, cmd: u8) -> &Self {
        self.hw.rs(false);
        self.hw.wait_address(); // tAS
        self.send(cmd);
        self.wait_ready_default();
        self
    }


    // Typical command wait time is 37us
    fn wait_ready_default(&self) {
        self.wait_ready(50);
    }

    fn wait_ready(&self, delay: u32) {
        self.hw.delay_us(delay);
    }

    fn pulse_enable(&self) {
        self.hw.enable(true);
        self.hw.delay_us(1); // minimum delay is 450 ns
        self.hw.enable(false);
    }

    fn send(&self, data: u8) {
        match self.hw.mode() {
            FunctionMode::Bit8 => {
                self.send_data(data);
            }
            FunctionMode::Bit4 => {
                self.send_data(data >> 4);
                self.send_data(data & 0xf);
            }
        }
    }

    fn send_data(&self, data: u8) {
        self.hw.data(data);
        self.pulse_enable();
    }
}

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::String;
    use std::vec::Vec;
    use std::cell::RefCell;

    struct StringHw {
        commands: RefCell<Vec<String>>,
        mode: FunctionMode,
    }

    impl StringHw {
        pub fn new(mode: FunctionMode) -> StringHw {
            StringHw {
                commands: RefCell::new(Vec::new()),
                mode
            }
        }

        pub fn commands(&self) -> Vec<String> {
            self.commands.borrow().clone()
        }
    }

    impl Hardware for StringHw {
        fn rs(&self, bit: bool) {
            self.commands.borrow_mut().push(format!("R/S {}", bit));
        }

        fn enable(&self, bit: bool) {
            self.commands.borrow_mut().push(format!("EN {}", bit));
        }

        fn data(&self, data: u8) {
            let str = match self.mode {
                FunctionMode::Bit4 => format!("DATA 0b{:04b}", data),
                FunctionMode::Bit8 => format!("DATA 0b{:08b}", data),
            };
            self.commands.borrow_mut().push(str);
        }

        fn mode(&self) -> FunctionMode {
            let mode = self.mode;
            mode
        }
    }

    impl Delay for StringHw {
        fn delay_us(&self, delay: u32) {
            self.commands.borrow_mut().push(format!("DELAY {}", delay));
        }
    }

    #[test]
    fn init_4bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.init(FunctionLine::Line2, FunctionDots::Dots5x8);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            // Send init command three times
            "R/S false",
            "DATA 0b0011",
            "EN true", "DELAY 1", "EN false", "DELAY 4500",
            "EN true", "DELAY 1", "EN false", "DELAY 150",
            "EN true", "DELAY 1", "EN false", "DELAY 50",
            // Switch to 4-bit mode
            "DATA 0b0010", "EN true", "DELAY 1", "EN false", "DELAY 50",
            // Set lines, font size
            "R/S false",
            "DATA 0b0010", "EN true", "DELAY 1", "EN false",
            "DATA 0b1000", "EN true", "DELAY 1", "EN false",
            "DELAY 50",
            // Display
            "R/S false",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b1000", "EN true", "DELAY 1", "EN false",
            "DELAY 50",
            // Clear
            "R/S false",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b0001", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 2000",
            // Entry mode
            "R/S false",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b0110", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);
    }

    #[test]
    fn init_8bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit8));
        lcd.init(FunctionLine::Line2, FunctionDots::Dots5x8);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            // Send init command three times
            "R/S false",
            "DATA 0b00111100",
            "EN true", "DELAY 1", "EN false", "DELAY 4500",
            "EN true", "DELAY 1", "EN false", "DELAY 150",
            "EN true", "DELAY 1", "EN false", "DELAY 50",
            // Set lines, font size
            "R/S false",
            "DATA 0b00111000", "EN true", "DELAY 1", "EN false",
            "DELAY 50",
            // Display
            "R/S false",
            "DATA 0b00001000", "EN true", "DELAY 1", "EN false",
            "DELAY 50",
            // Clear
            "R/S false",
            "DATA 0b00000001", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 2000",
            // Entry mode
            "R/S false",
            "DATA 0b00000110", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);
    }

    #[test]
    fn clear_4bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.clear();

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b0001", "EN true", "DELAY 1", "EN false",
            "DELAY 50",
            "DELAY 2000"
        ]);
    }

    #[test]
    fn clear_8bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit8));
        lcd.clear();

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b00000001", "EN true", "DELAY 1", "EN false",
            "DELAY 50",
            "DELAY 2000"
        ]);
    }

    #[test]
    fn home_4bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.home();

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b0010", "EN true", "DELAY 1", "EN false",
            "DELAY 50",
            "DELAY 2000"
        ]);
    }

    #[test]
    fn home_8bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit8));
        lcd.home();

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b00000010", "EN true", "DELAY 1", "EN false",
            "DELAY 50",
            "DELAY 2000"
        ]);
    }

    #[test]
    fn entry_mode_4bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.entry_mode(EntryModeDirection::EntryLeft, EntryModeShift::NoShift);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b0100", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);

        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::Shift);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b0111", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);
    }

    #[test]
    fn scroll_4bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.scroll(Direction::Left);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b0001", "EN true", "DELAY 1", "EN false",
            "DATA 0b1000", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);

        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.scroll(Direction::Right);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b0001", "EN true", "DELAY 1", "EN false",
            "DATA 0b1100", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);
    }

    #[test]
    fn cursor_4bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.cursor(Direction::Left);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b0001", "EN true", "DELAY 1", "EN false",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);

        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.cursor(Direction::Right);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b0001", "EN true", "DELAY 1", "EN false",
            "DATA 0b0100", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);
    }

    #[test]
    fn position_4bit() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.position(3, 0);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b1000", "EN true", "DELAY 1", "EN false",
            "DATA 0b0011", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);

        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.position(3, 1);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b1100", "EN true", "DELAY 1", "EN false",
            "DATA 0b0011", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);

        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.position(7, 2);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b1001", "EN true", "DELAY 1", "EN false",
            "DATA 0b1011", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);

        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.position(8, 3);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b1101", "EN true", "DELAY 1", "EN false",
            "DATA 0b1100", "EN true", "DELAY 1", "EN false",
            "DELAY 50"
        ]);
    }

    #[test]
    fn print() {
        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.print("hello");

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S true",
            "DATA 0b0110", "EN true", "DELAY 1", "EN false",
            "DATA 0b1000", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0110", "EN true", "DELAY 1", "EN false",
            "DATA 0b0101", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0110", "EN true", "DELAY 1", "EN false",
            "DATA 0b1100", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0110", "EN true", "DELAY 1", "EN false",
            "DATA 0b1100", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0110","EN true", "DELAY 1", "EN false",
            "DATA 0b1111", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5"
        ]);
    }

    #[test]
    fn upload() {
        const ARROW: [u8; 8] = [
            0b00000,
            0b01000,
            0b01100,
            0b01110,
            0b11111,
            0b01110,
            0b01100,
            0b01000
        ];

        let mut lcd = Display::new(StringHw::new(FunctionMode::Bit4));
        lcd.upload_character(3, ARROW);

        let vec = lcd.hw.commands();
        assert_eq!(vec, vec![
            "R/S false",
            "DATA 0b0101", "EN true", "DELAY 1", "EN false",
            "DATA 0b1000", "EN true", "DELAY 1", "EN false",
            "DELAY 50",
            "R/S true",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b1000", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b1100", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b1110", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0001", "EN true", "DELAY 1", "EN false",
            "DATA 0b1111", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b1110", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b1100", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
            "R/S true",
            "DATA 0b0000", "EN true", "DELAY 1", "EN false",
            "DATA 0b1000", "EN true", "DELAY 1", "EN false",
            "DELAY 50", "DELAY 5",
        ]);
    }
}

#![no_std]
#![allow(clippy::needless_doctest_main)]
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
//!     fn rs(&mut self, bit: bool) {
//!         // should set R/S pin on LCD screen
//!     }
//!     fn enable(&mut self, bit: bool) {
//!         // should set EN pin on LCD screen
//!     }
//!     fn data(&mut self, data: u8) {
//!         // should set data bits to the LCD screen (only lowest 4 bits are used in 4-bit mode).
//!     }
//!
//!     // optionally, override the following function to switch to 8-bit mode
//!     fn mode(&self) -> lcd::FunctionMode {
//!         lcd::FunctionMode::Bit8
//!     }
//!
//!     // optionally, implement the following three functions to enable polling busy flag instead of delay
//!     fn can_read(&self) -> bool {
//!         true
//!     }
//!
//!     fn rw(&mut self, bit: bool) {
//!         // configure pins for input _before_ setting R/W to 1
//!         // configure pins for output _after_ setting R/W to 0
//!     }
//!     fn read_data(&mut self) -> u8 {
//!         0 // read data from the port
//!     }
//! }
//!
//! // implement `Delay` trait to allow library to sleep for the given amount of time
//! impl Delay for HW {
//!     fn delay_us(&mut self, delay_usec: u32) {
//!         // should sleep for the given amount of microseconds
//!     }
//! }
//!
//! # fn main() {
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
//!
//! # }
//! ```
//!
//! See [`lcd-example-bluepill`](https://github.com/idubrov/lcd-example-bluepill) for the working example
//! for the [Blue Pill](http://wiki.stm32duino.com/index.php?title=Blue_Pill) development board.
//!
//! [1]: https://en.wikipedia.org/wiki/Hitachi_HD44780_LCD_controller

#[derive(Copy, Clone, Debug)]
pub enum FunctionMode {
    /// Send data 4 bits at the time
    Bit4 = 0x00,
    /// Send data 8 bits at the time
    Bit8 = 0x10,
}

#[derive(Copy, Clone, Debug)]
pub enum FunctionDots {
    Dots5x8 = 0x00,
    Dots5x10 = 0x04,
}

#[derive(Copy, Clone, Debug)]
pub enum FunctionLine {
    Line1 = 0x00,
    Line2 = 0x08,
}

#[derive(Copy, Clone, Debug)]
pub enum DisplayBlink {
    BlinkOff = 0x00,
    BlinkOn = 0x01,
}

#[derive(Copy, Clone, Debug)]
pub enum DisplayCursor {
    CursorOff = 0x00,
    CursorOn = 0x02,
}

#[derive(Copy, Clone, Debug)]
pub enum DisplayMode {
    DisplayOff = 0x00,
    DisplayOn = 0x04,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Left = 0x00,
    Right = 0x04,
}

#[derive(Copy, Clone, Debug)]
pub enum Scroll {
    CursorMove = 0x00,
    DisplayMove = 0x08,
}

#[derive(Copy, Clone, Debug)]
pub enum EntryModeDirection {
    EntryLeft = 0x00,
    EntryRight = 0x02,
}

#[derive(Copy, Clone, Debug)]
pub enum EntryModeShift {
    NoShift = 0x00,
    Shift = 0x01,
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
    SetDDRamAddr = 0x80,
}

pub trait Delay {
    /// Delay for given amount of time (in microseconds).
    fn delay_us(&mut self, delay_usec: u32);
}

trait InternalHardware {
    fn rs(&self, bit: bool);
    fn enable(&self, bit: bool);
    fn data(&self, data: u8);
    fn wait_address(&self) {}
    fn mode(&self) -> FunctionMode;
    fn rw(&self, bit: bool);
    fn read_data(&self) -> u8;
}

pub trait Hardware {
    fn rs(&mut self, bit: bool);
    fn enable(&mut self, bit: bool);
    fn data(&mut self, data: u8);

    /// Address set up time is 40ns minimum (tAS)
    /// This function should be overridden in case processor is too fast for 40ns to pass.
    fn wait_address(&mut self) {}

    /// Override to pick 8-bit mode (4-bit mode by default)
    fn mode(&self) -> FunctionMode {
        FunctionMode::Bit4
    }

    /// If this implementation can read from the data port. Default is `false`. If `true` is
    /// returned, both `rw` and `read_data` need to be implemented.
    fn can_read(&self) -> bool {
        false
    }

    /// Set R/W flag.
    ///
    /// Implementation should re-configure GPIO for input _before_ setting R/W pin to `true`
    /// and configure GPIO for output _after_ setting R/W to `false`.
    ///
    /// Note that LCD driver typically uses 5V, so input should be tolerant to 5V when using busy
    /// flag.
    ///
    /// Default implementation will panic.
    fn rw(&mut self, _bit: bool) {
        unimplemented!()
    }

    /// Read data from the data pins of the LCD (D0-D7 in 8-bit mode and D4-D7 in 4-bit mode)
    ///
    /// Default implementation will panic.
    fn read_data(&mut self) -> u8 {
        unimplemented!()
    }

    /// Send data to the device.
    ///
    /// This is mainly for LCDs attached via I2C / SMBUS where it's important to make changes to
    /// data and control lines at the same time.
    ///
    /// If control and data lines are directly attached, there's no need to implement this method.
    fn apply(&mut self) {}
}

/// Object implementing HD44780 protocol. Stateless (could be created as many times as needed).
pub struct Display<HW: Hardware + Delay> {
    hw: HW,
}

trait WaitReady {
    fn wait_ready(&self, delay: u32);
}

impl<HW: Hardware + Delay> core::fmt::Write for Display<HW> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl<HW: Hardware + Delay> Display<HW> {
    pub fn new(hw: HW) -> Display<HW> {
        Display { hw }
    }

    /// Initialize LCD display. Sets an equivalent of the following setup:
    ///
    /// ```rust,no_run
    /// # use lcd::*;
    /// # struct HW {}
    /// # impl Hardware for HW {
    /// #   fn rs(&mut self, bit: bool) { }
    /// #   fn enable(&mut self, bit: bool) { }
    /// #   fn data(&mut self, data: u8) { }
    /// # }
    /// # impl Delay for HW {
    /// #   fn delay_us(&mut self, delay_usec: u32) { }
    /// # }
    /// # let hw = HW {};
    /// # let mut lcd = Display::new(hw);
    /// lcd.display(DisplayMode::DisplayOff, DisplayCursor::CursorOff, DisplayBlink::BlinkOff);
    /// lcd.clear();
    /// lcd.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::NoShift);
    /// ```
    #[inline(never)]
    pub fn init(&mut self, line: FunctionLine, dots: FunctionDots) {
        let mode = self.hw.mode();
        self.hw.rs(false);
        self.hw.apply();
        self.hw.wait_address();
        match mode {
            FunctionMode::Bit8 => {
                // Run initialization procedure for the display (8-bit mode).

                // Set to 8-bit mode, 2 line, 5x10 font
                // Display off, clear, entry mode set
                self.send_data(
                    (Command::FunctionSet as u8)
                        | (FunctionMode::Bit8 as u8)
                        | (FunctionLine::Line2 as u8)
                        | (FunctionDots::Dots5x10 as u8),
                ); // Send command for the first time

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
        self.display(
            DisplayMode::DisplayOff,
            DisplayCursor::CursorOff,
            DisplayBlink::BlinkOff,
        );
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
    pub fn display(
        &mut self,
        display: DisplayMode,
        cursor: DisplayCursor,
        blink: DisplayBlink,
    ) -> &Self {
        self.command(
            (Command::DisplayControl as u8) | (display as u8) | (cursor as u8) | (blink as u8),
        )
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
            _ => 0,
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
    #[inline(never)]
    pub fn write(&mut self, data: u8) -> &Self {
        self.hw.rs(true);
        self.hw.apply();
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
    #[inline(never)]
    pub fn upload_character(&mut self, location: u8, map: [u8; 8]) -> &Self {
        assert!(location <= 7);

        // Only 8 locations are available
        self.command((Command::SetCGRamAddr as u8) | ((location & 0x7) << 3));
        for item in map.iter().take(8) {
            self.write(*item);
        }
        self
    }

    #[inline(never)]
    fn command(&mut self, cmd: u8) -> &Self {
        self.hw.rs(false);
        self.hw.apply();
        self.hw.wait_address(); // tAS
        self.send(cmd);
        self.wait_ready_default();
        self
    }

    // Typical command wait time is 37us
    fn wait_ready_default(&mut self) {
        self.wait_ready(50);
    }

    #[inline(never)]
    fn pulse_enable(&mut self) {
        self.hw.enable(true);
        self.hw.apply();
        self.hw.delay_us(1); // minimum delay is 450 ns
        self.hw.enable(false);
        self.hw.apply();
    }

    #[inline(never)]
    fn send(&mut self, data: u8) {
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

    #[inline(never)]
    fn send_data(&mut self, data: u8) {
        self.hw.data(data);
        self.hw.apply();
        self.pulse_enable();
    }

    /// Function to wait until HD44780 is ready.
    #[inline(never)]
    fn wait_ready(&mut self, delay: u32) {
        if self.hw.can_read() {
            self.hw.rs(false);

            // Read mode
            self.hw.rw(true);
            self.hw.apply();
            self.hw.wait_address(); // tAS

            while self.receive() & 0b1000_0000 != 0 {}
            // tAH is 10ns, which is less than one cycle. So we don't have to wait.

            // Back to write mode
            self.hw.rw(false);
            self.hw.apply();
        } else {
            // Cannot read "ready" flag, so do a delay.
            self.hw.delay_us(delay);
        }
    }

    #[inline(never)]
    fn receive_data(&mut self) -> u8 {
        self.hw.enable(true);
        self.hw.apply();
        self.hw.delay_us(1);
        let data = self.hw.read_data();
        self.hw.delay_us(1);
        self.hw.enable(false);
        self.hw.apply();
        data
    }

    fn receive(&mut self) -> u8 {
        match self.hw.mode() {
            FunctionMode::Bit8 => self.receive_data(),
            FunctionMode::Bit4 => (self.receive_data() << 4) | (self.receive_data() & 0xf),
        }
    }

    /// Unwrap HAL back from the driver.
    pub fn unwrap(self) -> HW {
        self.hw
    }
}

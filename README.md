[![crates.io](https://img.shields.io/crates/v/lcd.svg)](https://crates.io/crates/lcd)
[![crates.io](https://img.shields.io/crates/d/lcd.svg)](https://crates.io/crates/lcd)
[![CircleCI](https://img.shields.io/circleci/project/github/idubrov/lcd.svg)](https://circleci.com/gh/idubrov/lcd)
[![Codecov](https://img.shields.io/codecov/c/github/idubrov/lcd.svg)](https://codecov.io/gh/idubrov/lcd)

# lcd

Library that implements low-level protocol to the [Hitachi HD44780][1]-compatible LCD device.

Provides high-level API to the [Hitachi HD44780][1]-compatible LCD device. Uses 4-bit mode
by default (only uses 4 data pins) plus two control pins (R/S and EN). Using the R/W pin is optional;
when `Hardware::can_read()` returns `false` (the default implementation), it is not used and should be
wired for "write" (low-level, 0).

The implementation is completely stateless. Client is free to reuse the same `Display` object
or to create one every time access to LCD is required.

`Display` also implements `core::fmt::Write` trait, so it could be used as a target of `write!`
macro.

This library does not depend on `std` crate and could be used in bare metal embedded development.

## Examples
```rust
use core::fmt::Write; // for write!
use lcd::*;

// implement HAL...
struct HW {
    // any data needed to access low-level peripherals
}

// Implement the `Hardware` trait to give access to LCD pins
impl Hardware for HW {
    fn rs(&mut self, bit: bool) {
        // should set R/S pin on LCD screen
    }
    fn enable(&mut self, bit: bool) {
        // should set EN pin on LCD screen
    }
    fn data(&mut self, data: u8) {
        // should set data bits to the LCD screen (only lowest 4 bits are used in 4-bit mode).
    }

    // optionally, override the following function to switch to 8-bit mode
    fn mode(&self) -> lcd::FunctionMode {
        lcd::FunctionMode::Bit8
    }

    // optionally, implement the following three functions to enable polling busy flag instead of delay
    fn can_read(&self) -> bool {
        true
    }

    fn rw(&mut self, bit: bool) {
        // configure pins for input _before_ setting R/W to 1
        // configure pins for output _after_ setting R/W to 0
    }
    fn read_data(&mut self) -> u8 {
        0 // read data from the port
    }
}

// Implement the `Delay` trait to allow library to sleep for the given amount of time
impl Delay for HW {
    fn delay_us(&mut self, delay_usec: u32) {
        // should sleep for the given amount of microseconds
    }
}

// If your hardware has a backlight, implement `Backlight` trait to control it
impl Backlight for HW {
    fn set_backlight(&mut self, enable: bool) {
        // configure pins to turn the backlight on/off.
    }
}


// create HAL and LCD instances
let hw = HW { /* ... */ };
let mut lcd = Display::new(hw);

// initialization
lcd.init(FunctionLine::Line2, FunctionDots::Dots5x8);
lcd.display(
    DisplayMode::DisplayOn,
    DisplayCursor::CursorOff,
    DisplayBlink::BlinkOff);
lcd.backlight(true); // available only if HW implements Backlight.
lcd.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::NoShift);

// print something
write!(&mut lcd, "Hello, my number today is {: >4}", 42).unwrap();

```

See [`lcd-example-bluepill`](https://github.com/idubrov/lcd-example-bluepill) for the working example
for the [Blue Pill](http://wiki.stm32duino.com/index.php?title=Blue_Pill) development board.

[1]: https://en.wikipedia.org/wiki/Hitachi_HD44780_LCD_controller

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[![crates.io](https://img.shields.io/crates/v/lcd.svg)](https://crates.io/crates/lcd)
[![crates.io](https://img.shields.io/crates/d/lcd.svg)](https://crates.io/crates/lcd)
[![CircleCI](https://img.shields.io/circleci/project/github/idubrov/lcd.svg)]()

# lcd

Library that implements low-level protocol to the [Hitachi HD44780][1]-compatible LCD device.

Provides high-level API to the [Hitachi HD44780][1]-compatible LCD device. Uses 4-bit mode
by default (only uses 4 data pins) plus two control pins (R/S and EN). R/W pin is not used
and should be wired for "write" (low-level, 0).

The implementation is completely stateless. Client is free to reuse the same `HD44780` object
or to create one every time access to LCD is required.

`HD44780` also implements `core::fmt::Write` trait, so it could be used as a target of `write!`
macro.

This library does not depend on `std` crate and could be used in bare metal embedded development.

## Examples
```rust
#![no_std]
use core::fmt::Write; // for write!
use lcd::*;

// implement HAL...
struct HW {
    // any data needed to access low-level peripherals
}

// implement `Hardware` trait to give access to LCD pins
impl Hardware for HW {
    fn rs(&self, bit: bool) {
        // should set R/S pin on LCD screen
    }
    fn enable(&self, bit: bool) {
        // should set EN pin on LCD screen
    }
    fn data(&self, data: u8) {
        // should set data bits to the LCD screen (only lowest 4 bits are used in 4-bit mode).
    }

    // optionally, override the following function to switch to 8-bit mode
    fn mode(&self) -> lcd::FunctionMode {
        lcd::FunctionMode::Bit8
    }
}

// implement `Delay` trait to allow library to sleep for given amount of time
impl Delay for HW {
    fn delay_us(&self, delay_usec: u32) {
        // should delay for the given amount of microseconds
    }
}

// create HAL and LCD instances
let hw = HW { /* ... */ };
let mut lcd = HD44780::new(hw);

// initialization
lcd.init();
lcd.display(
    DisplayMode::DisplayOn,
    DisplayCursor::CursorOff,
    DisplayBlink::BlinkOff);
lcd.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::NoShift);

// print something
write!(&mut lcd, "Hello, my number today is {: >4}", 42).unwrap();
```

### Links
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

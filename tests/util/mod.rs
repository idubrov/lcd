use lcd::*;
use std::string::String;
use std::vec::Vec;

pub struct BufferHardware {
    pub commands: Vec<String>,
    pub input: Option<Vec<u8>>,
    pub mode: FunctionMode,
}

impl BufferHardware {
    pub fn command(&mut self, cmd: String) {
        self.commands.push(cmd);
    }
}

impl Hardware for BufferHardware {
    fn rs(&mut self, bit: bool) {
        self.command(format!("R/S {}", bit));
    }

    fn enable(&mut self, bit: bool) {
        self.command(format!("EN {}", bit));
    }

    fn data(&mut self, data: u8) {
        let str = match self.mode {
            FunctionMode::Bit4 => format!("DATA 0b{:04b}", data),
            FunctionMode::Bit8 => format!("DATA 0b{:08b}", data),
        };
        self.command(str);
    }

    fn mode(&self) -> FunctionMode {
        self.mode
    }

    fn can_read(&self) -> bool {
        self.input.is_some()
    }

    fn rw(&mut self, bit: bool) {
        self.command(format!("RW {}", bit));
    }

    fn read_data(&mut self) -> u8 {
        self.command("IS BUSY?".to_string());
        self.input.as_mut().unwrap().remove(0)
    }
}

impl Delay for BufferHardware {
    fn delay_us(&mut self, delay: u32) {
        self.command(format!("DELAY {}", delay));
    }
}

pub struct IgnoredDelay;

impl Delay for IgnoredDelay {
    fn delay_us(&mut self, _delay: u32) {}
}

impl Backlight for BufferHardware {
    fn set_backlight(&mut self, enable: bool) {
        self.command(format!("BACKLIGHT {}", enable));
    }
}

pub fn test(
    mode: FunctionMode,
    input: Option<Vec<u8>>,
    ops: impl Fn(&mut Display<BufferHardware>),
) -> Vec<String> {
    let hw = BufferHardware {
        commands: vec![],
        input,
        mode,
    };
    let mut display = Display::new(hw);
    ops(&mut display);
    display.unwrap().commands
}

/// Tests against the HardwareDelay combiner, using an IgnoredDelay implementation.
#[allow(dead_code)] // false warning
pub fn test_ignored_delay(
    mode: FunctionMode,
    input: Option<Vec<u8>>,
    ops: impl Fn(&mut Display<HardwareDelay<BufferHardware, IgnoredDelay>>),
) -> Vec<String> {
    let hw = BufferHardware {
        commands: vec![],
        input,
        mode,
    };
    let delay = IgnoredDelay;
    let mut display = Display::new(HardwareDelay::new(hw, delay));
    ops(&mut display);
    display.unwrap().unwrap().0.commands
}

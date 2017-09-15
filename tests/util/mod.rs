use lcd::*;
use std::string::String;
use std::vec::Vec;
use std::cell::RefCell;

pub struct BufferHardware<'a, T: 'a> {
    pub commands: &'a RefCell<Vec<String>>,
    pub mode: FunctionMode,
    pub busy: RefCell<T>
}

impl<'a, T> BufferHardware<'a, T> {
    pub fn command(&self, cmd: String) {
        self.commands.borrow_mut().push(cmd);
    }
}

impl<'a, T> Hardware for BufferHardware<'a, T> {
    fn rs(&self, bit: bool) {
        self.command(format!("R/S {}", bit));
    }

    fn enable(&self, bit: bool) {
        self.command(format!("EN {}", bit));
    }

    fn data(&self, data: u8) {
        let str = match self.mode {
            FunctionMode::Bit4 => format!("DATA 0b{:04b}", data),
            FunctionMode::Bit8 => format!("DATA 0b{:08b}", data),
        };
        self.command(str);
    }

    fn mode(&self) -> FunctionMode {
        let mode = self.mode;
        mode
    }
}

impl<'a, T> Delay for BufferHardware<'a, T> {
    fn delay_us(&self, delay: u32) {
        self.command(format!("DELAY {}", delay));
    }
}

pub fn test<F, T>(mode: FunctionMode, busy: T, ops: F) -> Vec<String>
    where F: Fn(&mut Display<BufferHardware<T>>) {
    let commands = RefCell::new(Vec::new());
    ops(&mut Display::new(BufferHardware {
        commands: &commands,
        mode,
        busy: RefCell::new(busy)
    }));
    let result = commands.borrow().clone();
    result
}
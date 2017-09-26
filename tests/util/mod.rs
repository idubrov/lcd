use lcd::*;
use std::string::String;
use std::vec::Vec;
use std::cell::RefCell;

pub struct BufferHardware<T> {
    pub commands: RefCell<Vec<String>>,
    pub mode: FunctionMode,
    pub busy: RefCell<T>
}

impl<T> BufferHardware<T> {
    pub fn command(&self, cmd: String) {
        self.commands.borrow_mut().push(cmd);
    }
}

impl<T> Hardware for BufferHardware<T> {
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

impl<'a, T> Delay for BufferHardware<T> {
    fn delay_us(&self, delay: u32) {
        self.command(format!("DELAY {}", delay));
    }
}

pub fn test<F, T>(mode: FunctionMode, busy: T, ops: F) -> Vec<String>
    where F: Fn(&mut Display<BufferHardware<T>>) {
    let hw = BufferHardware {
        commands: RefCell::new(Vec::new()),
        mode,
        busy: RefCell::new(busy)
    };
    ops(&mut Display::new(&hw));
    let result = hw.commands.borrow().clone();
    result
}
use lcd::*;
use std::string::String;
use std::vec::Vec;
use std::cell::RefCell;

pub struct StringHw<'a> {
    commands: &'a RefCell<Vec<String>>,
    mode: FunctionMode,
}

impl <'a> Hardware for StringHw<'a> {
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

impl <'a> Delay for StringHw<'a> {
    fn delay_us(&self, delay: u32) {
        self.commands.borrow_mut().push(format!("DELAY {}", delay));
    }
}

pub fn test<F>(mode: FunctionMode, ops: F) -> Vec<String>
    where F: Fn(&mut Display<StringHw>) {
    let commands = RefCell::new(Vec::new());
    let hw = StringHw {
        commands: &commands,
        mode
    };
    let mut lcd = Display::new(hw);
    ops(&mut lcd);
    let result = commands.borrow().clone();
    result
}
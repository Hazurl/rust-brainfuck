use super::instruction::*;

use std::fmt;
use std::io;
use std::io::Read;

pub struct Memory {
    memory: Vec<u8>,
    ptr: usize,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            memory: vec![0; size],
            ptr: 0,
        }
    }

    pub fn value(&mut self, offset: i32) -> &mut u8 {
        &mut self.memory[Memory::check_ptr_overflow(self.ptr, offset)]
    }

    pub fn add(&mut self, amount: i32, offset: i32) {
        let value = *self.value(offset) as i32 + amount;

        *self.value(offset) = Memory::check_value_overflow(value);
    }

    pub fn set(&mut self, value: u8, offset: i32) {
        *self.value(offset) = value
    }

    pub fn move_ptr(&mut self, offset: i32) {
        self.ptr = Memory::check_ptr_overflow(self.ptr, offset);
    }

    pub fn set_ptr(&mut self, ptr: usize) {
        self.ptr = ptr
    }

    fn check_value_overflow(value: i32) -> u8 {
        const MAX: i32 = u8::max_value() as i32;
        const MIN: i32 = u8::min_value() as i32;
        return if value > MAX {
            value % MAX
        } else if value < MIN {
            MAX - (-value % MAX)
        } else {
            value
        } as u8;
    }

    pub fn check_ptr_overflow(ptr: usize, offset: i32) -> usize {
        const MAX: i64 = usize::max_value() as i64;
        const MIN: i64 = usize::min_value() as i64;

        let p = ptr as i64 + offset as i64;
        return if p > MAX {
            p % MAX
        } else if p < MIN {
            MAX - (-p % MAX)
        } else {
            p
        } as usize;
    }
}

pub struct State {
    memory: Memory,
    ptr_code: usize,
}

impl State {
    pub fn new(size: usize) -> State {
        State {
            memory: Memory::new(size),
            ptr_code: 0,
        }
    }

    fn read() -> u8 {
        let mut ipt = [0u8; 1];
        io::stdin().read_exact(&mut ipt).expect("Input error!");
        ipt[0]
    }

    fn write(value: u8) {
        println!("{}", value as char);
    }

    fn execute_once(&mut self, instr: Instruction) {
        match instr {
            Instruction::Add(x) => self.memory.add(x, 0),
            Instruction::Move(x) => self.memory.move_ptr(x),
            Instruction::Write => State::write(*self.memory.value(0)),
            Instruction::Read => self.memory.set(State::read(), 0),
            Instruction::Clear => self.memory.set(0, 0),
            Instruction::Copy(x) => {
                let value = *self.memory.value(x);
                self.memory.set(value, 0)
            }
            Instruction::JumpZero(x) => if *self.memory.value(0) == 0 {
                self.ptr_code = x;
            },
            Instruction::JumpNonZero(x) => if *self.memory.value(0) != 0 {
                self.ptr_code = x;
            },
        }

        self.ptr_code += 1;
    }

    pub fn execute(&mut self, instrs: Vec<Instruction>) {
        for i in instrs {
            self.execute_once(i);
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "" /*"[{}, {}, {}, {}, {}] around {}"
            , self.memory[(self.ptr + SIZE - 2) % SIZE]
            , self.memory[(self.ptr + SIZE - 1) % SIZE]
            , self.memory[(self.ptr           ) % SIZE]
            , self.memory[(self.ptr        + 1) % SIZE]
            , self.memory[(self.ptr        + 2) % SIZE]
            , self.ptr*/
        )
    }
}

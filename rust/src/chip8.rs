use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::ptr;

const REGISTER_COUNT : usize = 16;
const MEMORY_SIZE : usize = 4096;
const SCREEN_HEIGHT : usize = 32;
const STACK_SIZE : usize = 16;

pub struct Chip8 {
    registers: [u8; REGISTER_COUNT],
    memory: [u8; MEMORY_SIZE],
    screen: [u64; SCREEN_HEIGHT],
    stack: [u16; STACK_SIZE],
    delay_timer: u8,
    sound_timer: u8,
    opcode: u16,
    key: u8,
    I: u16,
    PC: u16,
    SP: u16,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            registers: [0; REGISTER_COUNT],
            memory: [0; MEMORY_SIZE],
            screen: [0; SCREEN_HEIGHT],
            stack: [0; STACK_SIZE],
            opcode: 0,
            delay_timer: 0, sound_timer: 0,
            I: 0, PC: 0, SP: 0, key: 0,
        }
    }

    pub fn load_file(&mut self, filename: &str) -> std::io::Result<()> {
        let len = fs::metadata(filename)?.len() as usize;
        let mut f = File::open(filename)?;

        self.clear();
        f.read_exact(&mut self.memory[0x200..(0x200 + len)])?;
        Ok(())
    }

    pub fn play(&self) {

    }

    fn clear(&mut self) {
        unsafe {
            let reg_ptr = self.registers.as_mut_ptr();
            ptr::write_bytes(reg_ptr, 0, REGISTER_COUNT);

            let mem_ptr = self.memory.as_mut_ptr();
            ptr::write_bytes(mem_ptr, 0, MEMORY_SIZE);

            let screen_ptr = self.screen.as_mut_ptr();
            ptr::write_bytes(screen_ptr, 0, SCREEN_HEIGHT);

            let stack_ptr = self.stack.as_mut_ptr();
            ptr::write_bytes(stack_ptr, 0, STACK_SIZE);
        }
    }
}

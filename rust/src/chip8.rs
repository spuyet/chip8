use std::io::prelude::*;
use std::fs::File;

const REGISTER_COUNT: usize = 16;
const MEMORY_SIZE : usize = 4096;
const SCREEN_HEIGHT : usize = 32;
const STACK_SIZE: usize = 16;

pub struct Chip8 {
    registers: [u8; REGISTER_COUNT],
    memory: [u8; MEMORY_SIZE],
    screen: [u64; SCREEN_HEIGHT],
    stack: [u16; STACK_SIZE],
    delay_timer: u8,
    sound_timer: u8,
    opcode: u16,
    game: Vec<u8>,
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
            opcode: 0, game: vec![],
            delay_timer: 0, sound_timer: 0,
            I: 0, PC: 0, SP: 0, key: 0,
        }
    }

    pub fn load_file(&mut self, filename: &str) -> std::io::Result<()> {
        let mut f = File::open(filename)?;
        f.read_to_end(&mut self.game)?;
        Ok(())
    }

    pub fn play(&self) {

    }
}

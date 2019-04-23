use std::io::prelude::Read;
use std::fs::File;
use std::fs;
use std::ptr;

const MEMORY_SIZE : usize = 4096;
const SCREEN_HEIGHT : usize = 32;
const SPRITES : [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x80, 0x80, 0x80, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

mod cpu;
use self::cpu::Cpu;

pub struct Chip8 {
    cpu: Cpu,
    memory: [u8; MEMORY_SIZE],
    screen: [u64; SCREEN_HEIGHT],
    delay_timer: u8,
    sound_timer: u8,
    key: u8,
    running: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            memory: [0; MEMORY_SIZE],
            screen: [0; SCREEN_HEIGHT],
            delay_timer: 0, sound_timer: 0, key: 0,
            running: false,
        }
    }

    pub fn load_file(&mut self, filename: &str) -> std::io::Result<()> {
        let len = fs::metadata(filename)?.len() as usize;
        let mut f = File::open(filename)?;

        self.clear();
        f.read_exact(&mut self.memory[0x200..(0x200 + len)])?;
        Ok(())
    }

    pub fn play(&mut self) {
        self.run();
    }

    pub fn pause(&self) {

    }

    pub fn stop(&self) {

    }

    fn run(&mut self) {
        self.running = true;
        loop {
            self.cpu.step(&mut self.memory, &mut self.screen);
        }
        self.running = false;
    }

    fn clear(&mut self) {
        self.cpu.clear();

        unsafe {
            let mem_ptr = self.memory.as_mut_ptr();
            ptr::write_bytes(mem_ptr, 0, MEMORY_SIZE);

            let screen_ptr = self.screen.as_mut_ptr();
            ptr::write_bytes(screen_ptr, 0, SCREEN_HEIGHT);
        }
    }
}

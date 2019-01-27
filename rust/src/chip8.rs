use std::io::prelude::Read;
use std::fs::File;
use std::fs;
use std::ptr;

const MEMORY_SIZE : usize = 4096;
const SCREEN_HEIGHT : usize = 32;

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
            self.cpu.step(&self.memory, &mut self.screen);
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

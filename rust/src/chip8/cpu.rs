use std::ptr;

const REGISTER_COUNT : usize = 16;
const STACK_SIZE : usize = 16;

pub struct Cpu {
    registers: [u8; REGISTER_COUNT],
    stack: [u16; STACK_SIZE],
    opcode: u16, i: u16,
    sp: u16, pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            registers: [0; REGISTER_COUNT],
            stack: [0; STACK_SIZE],
            opcode: 0, i: 0, sp: 0, pc: 0x200
        };
        cpu.clear();
        cpu
    }

    pub fn clear(&mut self) {
        self.pc = 0x200;
        unsafe {
            let reg_ptr = self.registers.as_mut_ptr();
            ptr::write_bytes(reg_ptr, 0, REGISTER_COUNT);

            let stack_ptr = self.stack.as_mut_ptr();
            ptr::write_bytes(stack_ptr, 0, STACK_SIZE);
        }
    }

    pub fn step(&mut self, memory: &mut [u8], screen: &mut [u64]) {
        let pc = self.pc as usize;
        let opcode = &memory[pc..(pc+2)];

        println!("{:x?}", opcode);
        match opcode[0] >> 4 {
            0x0 => println!("0x0 => ignored"),                                                                      // 0nnn - SYS addr
            0x1 => println!("0x1"),
            0x2 => {                                                                                                // 2nnn - CALL addr
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                let mut pc = ((opcode[0] & 0xF) as u16) << 8;
                pc |= opcode[1] as u16;
                self.pc = pc;
                return
            }
            0x3 => println!("0x3"),
            0x4 => println!("0x4"),
            0x5 => println!("0x5"),
            0x6 => self.registers[(opcode[0] & 0xF) as usize] = opcode[1],                                          // 6xkk - LD Vx, byte
            0x7 => self.registers[(opcode[0] & 0xF) as usize] += opcode[1],                                         // 7xkk - ADD Vx, byte
            0x8 => {
                match opcode[1] & 0xF {
                    0x0 => self.registers[(opcode[0] & 0xF) as usize] = self.registers[(opcode[1] >> 4) as usize], // 8xy0 - LD Vx, Vy
                    _ => ()
                }
            }
            0x9 => println!("0x9"),
            0xA => {                                                                                               // Annn - LD I, addr
                self.i = ((opcode[0] & 0xF) as u16) << 8;
                self.i |= opcode[1] as u16;
            }
            0xB => println!("0xB"),
            0xC => println!("0xC"),
            0xD => self.screen_update(opcode, memory, screen),                                                     // Dxyn - DRW Vx, Vy, nibble
            0xE => println!("0xE"),
            0xF => self.fx_instruction(opcode[1], opcode[0] & 0xF, memory),
            _ => ()
        }
        println!("registers: {:x?}", self.registers);
        println!("stack: {:x?}", self.stack);
        println!("pc: {:x?}", self.pc);
        println!("I: {:x?}", self.i);
        println!("memory: {:x?}", memory);
        println!("\n");
        self.pc += 2;
    }

    fn fx_instruction(&mut self, instruction: u8, register: u8, memory: &mut [u8]) {
        match instruction {
            0x29 => self.i = (register * 5) as u16,
            0x33 => {
                let v = self.registers[register as usize];
                memory[self.i as usize] = (v / 100) as u8;
                memory[(self.i + 1) as usize] = (v / 10 % 10) as u8;
                memory[(self.i + 2) as usize] = (v % 100 % 10) as u8;
            },
            0x65 => {
                let mut count = self.i as usize;
                for i in 0..(register + 1) {
                    self.registers[i as usize] = memory[count];
                    count += 1;
                }
            }
            _ => ()
        }
    }

    fn screen_update(&mut self, opcode: &[u8], memory: &[u8], screen: &mut [u64]) {
        let x = self.registers[(opcode[0] & 0x0F) as usize];
        let y = self.registers[(opcode[1] >> 4) as usize];
        let n = (opcode[1] & 0x0F) as usize;

        let mut j = self.i as usize;
        let mut update = 0;
        for i in (y as usize)..((y as usize) + n) {
            let v = memory[j] as u64;
            let before = screen[i];
            screen[i] ^= v << ((7 - x % 8) * 8);
            if before != screen[i] {
                update = 1;
            }
            j += 1;
        }
        self.registers[0xF] = update;
    }
}

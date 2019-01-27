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

    pub fn step(&mut self, memory: &[u8], screen: &mut [u64]) {
        let pc = self.pc as usize;
        let opcode = &memory[pc..(pc+2)];

        println!("{:x?}", opcode);
        match opcode[0] >> 4 {
            0x0 => println!("0x0"),
            0x1 => println!("0x1"),
            0x2 => {
                println!("0x2");
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                let mut pc = ((opcode[0] & 0x0F) as u16) << 8;
                pc |= opcode[1] as u16;
                self.pc = pc;
            }
            0x3 => println!("0x3"),
            0x4 => println!("0x4"),
            0x5 => println!("0x5"),
            0x6 => self.registers[(opcode[0] & 0x0F) as usize] = opcode[1],         // 6xkk - LD Vx, byte
            0x7 => println!("0x7"),
            0x8 => println!("0x8"),
            0x9 => println!("0x9"),
            0xA => {                                                               // LD I, addr
                self.i = ((opcode[0] & 0xF) as u16) << 8;
                self.i |= opcode[1] as u16;
            }
            0xB => println!("0xB"),
            0xC => println!("0xC"),
            0xD => self.screen_update(opcode, memory, screen),
            0xE => println!("0xE"),
            0xF => println!("0xF"),
            _ => ()
        }
        println!("{:?}", self.registers);
        println!("{:x}", self.i);
        self.pc += 2;
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

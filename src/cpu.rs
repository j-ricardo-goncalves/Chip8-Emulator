use crate::screen;

pub struct Cpu {
    pub mem: [u8; 4096], // memory
    pub pc: u16,         // program counter
    pub i: u16,          // index register
    pub v: [u8; 16],     // variable registers
    pub stack: [u16; 16],// stack
    pub sp: u8,          // stack pointer 
    pub dt: u8,          // delay timer
    pub st: u8,          // sound timer
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            mem: [0; 4096],
            pc: 0x200,
            i: 0,
            v: [0; 16],
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn tick(&mut self, buffer: &mut [u32]) {
        let high_nibble = self.mem[self.pc as usize] as u16;
        let low_nibble  = self.mem[(self.pc+1) as usize] as u16;
        let opcode: u16 = high_nibble << 8 | low_nibble;
        
        self.pc += 2;

        self.execute(opcode, buffer)        
    }

    pub fn execute(&mut self, opcode: u16, buffer: &mut [u32]) {
        let n1 = (opcode & 0xF000) >> 12;
        let n2 = (opcode & 0x0F00) >> 8;
        let n3 = (opcode & 0x00F0) >> 4;
        let n4 = (opcode & 0x000F);

        match (n1, n2, n3, n4) {
            (0, 0, 0xE, 0) => buffer.fill(screen::OFF),                       // clear screen 
            (1, _, _, _) => self.pc = (opcode & 0x0FFF),                      // jump
            (6, _, _, _) => self.v[n2 as usize] = (opcode & 0x00FF) as u8,    // set v register
            (7, _, _, _) => self.v[n2 as usize] += (opcode & 0x00FF) as u8,   // add v register
            (0xA, _, _, _) => self.i = opcode & 0xFFF,                        // set i register
            (0xD, _, _, _) => self.draw(n2, n3, n4, buffer),
            _ => println!("No such opcode: {}", opcode),
        }
    }
}


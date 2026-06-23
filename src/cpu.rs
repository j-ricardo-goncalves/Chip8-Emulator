use crate::screen;

const FONT: [u8; 80] = [
	0xF0, 0x90, 0x90, 0x90, 0xF0,		// 0
	0x20, 0x60, 0x20, 0x20, 0x70,		// 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0,		// 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0,		// 3
	0x90, 0x90, 0xF0, 0x10, 0x10,		// 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0,		// 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0,		// 6
	0xF0, 0x10, 0x20, 0x40, 0x40,		// 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0,		// 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0,		// 9
	0xF0, 0x90, 0xF0, 0x90, 0x90,		// A
	0xE0, 0x90, 0xE0, 0x90, 0xE0,		// B
	0xF0, 0x80, 0x80, 0x80, 0xF0,		// C
	0xE0, 0x90, 0x90, 0x90, 0xE0,		// D
	0xF0, 0x80, 0xF0, 0x80, 0xF0,		// E
	0xF0, 0x80, 0xF0, 0x80, 0x80		// F
];

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
        let mut mem = [0u8; 4096];
        mem[0x000..0x050].copy_from_slice(&FONT); // load font into memory
        Cpu {
            mem,
            pc: 0x200,
            i: 0,
            v: [0; 16],
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn tick(&mut self, buffer: &mut [u32], input: &[bool; 16]) {
        let high_nibble = self.mem[self.pc as usize] as u16;
        let low_nibble  = self.mem[(self.pc+1) as usize] as u16;
        let opcode: u16 = high_nibble << 8 | low_nibble;
        
        self.pc += 2;

        self.execute(opcode, buffer, input)        
    }

    pub fn execute(&mut self, opcode: u16, buffer: &mut [u32], input: &[bool; 16]) {
        let n1 = (opcode & 0xF000) >> 12;
        let n2 = (opcode & 0x0F00) >> 8;
        let n3 = (opcode & 0x00F0) >> 4;
        let n4 = (opcode & 0x000F);

        match (n1, n2, n3, n4) {
            (0, 0, 0xE, 0) => buffer.fill(screen::OFF),                       // clear screen 
            (0, 0, 0xE, 0xE) => self.return_call(),
            (1, _, _, _) => self.pc = (opcode & 0x0FFF),                      // jump
            (2, _, _, _) => self.call(opcode & 0x0FFF),
            (3, _, _, _) => if self.v[n2 as usize] == (opcode & 0x00FF) as u8 {self.pc += 2;},
            (4, _, _, _) => if self.v[n2 as usize] != (opcode & 0x00FF) as u8 {self.pc += 2;},
            (5, _, _, 0) => if self.v[n2 as usize] == self.v[n3 as usize] {self.pc += 2;},
            (6, _, _, _) => self.v[n2 as usize] = (opcode & 0x00FF) as u8,    // set v register
            (7, _, _, _) => self.v[n2 as usize] = self.v[n2 as usize].wrapping_add((opcode & 0x00FF) as u8),   // add v register
            (8, _, _, 0) => self.v[n2 as usize] = self.v[n3 as usize],
            (8, _, _, 1) => self.v[n2 as usize] |= self.v[n3 as usize],
            (8, _, _, 2) => self.v[n2 as usize] &= self.v[n3 as usize],
            (8, _, _, 3) => self.v[n2 as usize] ^= self.v[n3 as usize],
            (8, _, _, 4) => {
                let (result, overflow) = self.v[n2 as usize].overflowing_add(self.v[n3 as usize]);
                self.v[n2 as usize] = result;
                self.v[0xF] = overflow as u8;
            }
            (8, _, _, 5) => {
                let borrow = self.v[n2 as usize] < self.v[n3 as usize];
                self.v[n2 as usize] = self.v[n2 as usize].wrapping_sub(self.v[n3 as usize]);
                self.v[0xF] = !borrow as u8;
            }
            (8, _, _, 6) => {
                let bit = self.v[n2 as usize] & 0x1;
                self.v[n2 as usize] >>= 1;
                self.v[0xF] = bit;
            }
            (8, _, _, 7) => {
                let borrow = self.v[n3 as usize] < self.v[n2 as usize];
                self.v[n2 as usize] = self.v[n3 as usize].wrapping_sub(self.v[n2 as usize]);
                self.v[0xF] = !borrow as u8;
            }
            (8, _, _, 0xE) => {
                let bit = (self.v[n2 as usize] >> 7) & 0x1;
                self.v[n2 as usize] <<= 1;
                self.v[0xF] = bit;
            }
            (9, _, _, 0) => if self.v[n2 as usize] != self.v[n3 as usize] {self.pc += 2;},
            (0xA, _, _, _) => self.i = opcode & 0xFFF,                         // set i register
            (0xB, _, _, _) => self.pc = (opcode & 0x0FFF) + self.v[0] as u16,
            (0xC, _, _, _) => self.v[n2 as usize] = (rand::random_range(0..=0xFF) & (opcode & 0x00FF)) as u8,
            (0xD, _, _, _) => self.draw(n2, n3, n4, buffer),
            (0xE, _, 9, 0xE) => if input[self.v[n2 as usize] as usize] {self.pc += 2;},
            (0xE, _, 0xA, 1) => if !input[self.v[n2 as usize] as usize] {self.pc += 2;},
            (0xF, _, 0, 7) => self.v[n2 as usize] = self.dt,
            (0xF, _, 1, 5) => self.dt = self.v[n2 as usize],
            (0xF, _, 1, 8) => self.st = self.v[n2 as usize],
            (0xF, _, 1, 0xE) => self.i = self.i.wrapping_add(self.v[n2 as usize] as u16),
            (0xF, _, 0, 0xA) => match input.iter().position(|&k| k) {
                Some(key) => self.v[n2 as usize] = key as u8,
                None => self.pc -= 2,
            },
            (0xF, _, 2, 9) => self.i = self.v[n2 as usize] as u16 * 5,
            (0xF, _, 3, 3) => self.decimal_conversion(n2),
            (0xF, _, 5, 5) => self.store_v_register(n2),
            (0xF, _, 6, 5) => self.load_v_register(n2),
            _ => println!("No such opcode: {}", opcode),
        }
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 { self.dt -= 1; }
        if self.st > 0 { self.st -= 1; }
    }

    pub fn return_call(&mut self) {
        self.sp -= 1; 
        self.pc = self.stack[self.sp as usize]
    }

    pub fn call(&mut self, destination: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = destination;
    }

    pub fn draw(&mut self, vx: u16, vy: u16, n: u16, buffer: &mut [u32]) {
        let mut x = self.v[vx as usize] & (screen::WIDTH - 1) as u8;
        let mut y = self.v[vy as usize] & (screen::HEIGHT - 1) as u8;
        self.v[0xF as usize] = 0;
        for i in 0..n {
            if y as usize + i as usize >= 32 {break;}
            let mut sprite_data = self.mem[(self.i + i) as usize];
            for j in 0..8 {
                if x as usize + j as usize >= 64 {continue;}
                let index = (y as usize+ i as usize) * 64 + (x as usize + j as usize);
                if sprite_data & 0x80 != 0 {
                    buffer[index] ^= screen::ON;
                    if buffer[index] == screen::OFF {
                        self.v[0xF] = 1; 
                    }
                }
                sprite_data <<= 1;
            }
        }
    }

    pub fn decimal_conversion(&mut self, vx: u16) {
        let x = self.v[vx as usize];
        self.mem[self.i as usize]     = x / 100;
        self.mem[self.i as usize + 1] = (x / 10) % 10;
        self.mem[self.i as usize + 2] = x % 10;
    }

    pub fn store_v_register(&mut self, x: u16) {    
        for reg in 0..=x {
            self.mem[self.i as usize + reg as usize] = self.v[reg as usize];
        }
    }

    pub fn load_v_register(&mut self, x: u16) {
        for reg in 0..=x {
            self.v[reg as usize] = self.mem[self.i as usize + reg as usize];
        }
    }
}

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
}

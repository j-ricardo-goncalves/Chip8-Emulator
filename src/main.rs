use minifb::Key;
use std::fs;
mod cpu;
mod screen;

fn main() {
    let mut cpu = cpu::Cpu::new();
    let mut buffer = [0; screen::WIDTH * screen::HEIGHT];
    let mut window = screen::new();
    
    let rom = fs::read("roms/IBM Logo.ch8").expect("failed to read ROM");
    for (i, byte) in rom.iter().enumerate() {
        cpu.mem[0x200 + i] = *byte;
    }
    
    while !window.is_key_down(Key::Escape) {
        cpu::Cpu::tick(&mut cpu, &mut buffer);
        screen::update_buffer(&mut window, &buffer);
    }
}

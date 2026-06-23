use std::time;
use minifb::Key;
use std::fs;
mod cpu;
mod screen;
mod input;

fn main() {
    let mut cpu = cpu::Cpu::new();
    let mut buffer = [0; screen::WIDTH * screen::HEIGHT];
    let mut input = [false; 16];
    let mut window = screen::new();
    
    let rom = fs::read("roms/IBM Logo.ch8").expect("failed to read ROM");
    for (i, byte) in rom.iter().enumerate() {
        cpu.mem[0x200 + i] = *byte;
    }
    
    while !window.is_key_down(Key::Escape) {
        input::get_input(&window ,&mut input);
        cpu::Cpu::tick(&mut cpu, &mut buffer, &input);
        screen::update_buffer(&mut window, &buffer);
        std::thread::sleep(time::Duration::from_millis(20));
    }
}

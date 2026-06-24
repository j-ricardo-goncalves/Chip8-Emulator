use std::time;
use minifb::Key;
use std::fs;
mod cpu;
mod screen;
mod input;

fn read_rom(cpu: &mut cpu::Cpu, rom_path: &String) {
    let rom = fs::read(rom_path).expect("failed to read ROM");
    for (i, byte) in rom.iter().enumerate() {
        cpu.mem[0x200 + i] = *byte;
    }
}

fn main() {
    let mut cpu = cpu::Cpu::new();
    let mut buffer = [0; screen::WIDTH * screen::HEIGHT];
    let mut input = [false; 16];
    let mut window = screen::new();
    let mut paused = false;
        
    let args: Vec<String> = std::env::args().collect();
    let rom_path = args.get(1).expect("Usage: Chip8-Emulator <rom_path>");
    read_rom(&mut cpu, rom_path);
    
    while !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
        }
        if window.is_key_pressed(Key::F5, minifb::KeyRepeat::No) {
            cpu = cpu::Cpu::new();
            buffer.fill(screen::OFF);
            read_rom(&mut cpu, rom_path);
        }
        
        input::get_input(&window, &mut input);

        if !paused {
            for _ in 0..10 {
                cpu.tick(&mut buffer, &input);
            }
            cpu.tick_timers();
        }
        
        screen::update_buffer(&mut window, &buffer);
        std::thread::sleep(time::Duration::from_millis(16)); // ~60fps
    }
}

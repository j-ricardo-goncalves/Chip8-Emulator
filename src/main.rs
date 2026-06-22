use minifb::Key;
mod cpu;
mod screen;

fn main() {
    let mut cpu = cpu::Cpu::new();
    let mut window = screen::new();
    cpu::Cpu::tick(&mut cpu);
    while !window.is_key_down(Key::Escape) {
        screen::update_buffer(&mut window, vec![0; screen::WIDTH * screen::HEIGHT]);
    }
}

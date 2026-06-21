mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::new();
    cpu::Cpu::tick(&mut cpu);
}

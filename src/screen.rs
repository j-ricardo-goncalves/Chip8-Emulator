use minifb::{Window, WindowOptions};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub fn new() -> Window {
    let mut window = Window::new(
        "Chip 8 Emulator/Interpreter",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window
}

pub fn update_buffer(window: &mut Window, buffer: Vec<u32>) {
    window
        .update_with_buffer(&buffer, WIDTH, HEIGHT)
        .unwrap();
}

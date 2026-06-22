use minifb::{Window, WindowOptions, Scale};

pub const ON: u32 = 0x00FF66;
pub const OFF: u32 = 0x00000;
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub fn new() -> Window {
    let mut window = Window::new(
        "Chip 8 Emulator/Interpreter",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X16,
            ..WindowOptions::default() 
        },
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

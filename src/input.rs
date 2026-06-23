use minifb::{Key, Window};

pub fn get_input(window: &Window, input: &mut [bool; 16]) {
    input.fill(false);
    window.get_keys().iter().for_each(|key|
        match key {
            Key::Key1 => input[0x1] = true,
            Key::Key2 => input[0x2] = true,
            Key::Key3 => input[0x3] = true,
            Key::Key4 => input[0xC] = true,

            Key::Q => input[0x4] = true,
            Key::W => input[0x5] = true,
            Key::E => input[0x6] = true,
            Key::R => input[0xD] = true,

            Key::A => input[0x7] = true,
            Key::S => input[0x8] = true,
            Key::D => input[0x9] = true,
            Key::F => input[0xE] = true,

            Key::Z => input[0xA] = true,
            Key::X => input[0x0] = true,
            Key::C => input[0xB] = true,
            Key::V => input[0xF] = true,
             
            Key::Space => println!("PRESSING SPACE YIPPIE"),
            _ => println!(""),
        }
    );
}

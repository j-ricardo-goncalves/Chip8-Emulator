use minifb::{Key, Window};

pub fn get_input(window: &Window, input: &mut [bool; 16]) {
    input.fill(false);
    window.get_keys().iter().for_each(|key|
        match key {
            Key::Key1 => input[0] = true,
            Key::Key2 => input[1] = true,
            Key::Key3 => input[2] = true,
            Key::Key4 => input[3] = true,
            
            Key::Q => input[4] = true,
            Key::W => input[5] = true,
            Key::E => input[6] = true,
            Key::R => input[7] = true,

            Key::A => input[8] = true,
            Key::S => input[9] = true,
            Key::D => input[10] = true,
            Key::F => input[11] = true,

            Key::Z => input[12] = true,
            Key::X => input[13] = true,
            Key::C => input[14] = true,
            Key::V => input[15] = true,
             
            Key::Space => println!("PRESSING SPACE YIPPIE"),
            _ => println!(""),
        }
    );
}

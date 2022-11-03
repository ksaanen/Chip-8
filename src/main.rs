use crate::chip8::Chip8;

mod chip8;
mod register;
mod memory;
mod time;

fn main() {
    let chip8: Chip8 = Chip8::new();

    let t = time::Time::new();
}

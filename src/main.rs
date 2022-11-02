use crate::chip8::Chip8;

mod chip8;
mod register;
mod memory;

fn main() {
    let Chip8 = Chip8::new();

    println!("Hello, world!");
}

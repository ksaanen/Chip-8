use std::time::Duration;

use crate::chip8::Chip8;

mod chip8;
mod register;
mod memory;
mod time;

fn main() {
    let chip8: Chip8 = Chip8::new();

    // Basic Cpu cycle timer
    let mut t = time::Time::new();
    while t.running {
        if t.time_passed() > Duration::from_secs(1) {
            chip8.cycle();
            t.reset();
        }
    }
}

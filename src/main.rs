use std::{time::Duration, iter::Cycle};

use crate::chip8::Chip8;
use timer::{self, Timer};
use chrono;

mod chip8;
mod register;
mod memory;

fn main() {
    let chip8: Chip8 = Chip8::new();

    // Timing clock cycle
    let t = Timer::new();
    t.schedule_repeating(chrono::Duration::seconds(1), move || {
        chip8.cycle();
    });
}

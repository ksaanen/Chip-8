use crate::chip8::Chip8;

mod chip8;
mod register;
mod memory;
mod time;

fn main() {
    let chip8: Chip8 = Chip8::new();

    // Basic Cpu cycle timer
    let mut t = time::time_millis(); // time
    let mut n = time::time_millis(); // now
    loop {
        n = time::time_millis();
        if (n - t) > 1000  {
            chip8.cycle();
            t = n;
        }
    }
}

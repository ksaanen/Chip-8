use std::default;

use crate::{register::{Register}, memory::Memory};

pub struct Chip8 {
  memory: Memory,
  register: Register,
}

impl Chip8 {

  pub fn new() -> Chip8 {
    return Chip8 {
      memory: Memory::new(),
      register: Register::new()
    };
  }

  /// Run a Cpu cycle
  pub fn cycle(&self) {
    self.memory.read_from_memory(0x00);

    println!("Tick");
  }

  fn eval_opcode(&self) {
    match &self.register.i {
        0x00 => {

        },
        default => {

        }
    }
  }

  /// Clear the display.
  fn cls() {

  }

  /// Return from a subroutine.
  /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
  fn ret() {
    
  }

  /// Jump to location nnn.
  /// The interpreter sets the program counter to nnn.
  fn jp(addr: u8) {

  }

  /// Call subroutine at nnn.
  /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
  fn call(addr: u8) {

  }

  /// Skip next instruction if Vx = kk.
  /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
  fn se(Vx: u8, byte: u8) {

  }

  /// Skip next instruction if Vx != kk.
  /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
  fn sne(Vx: u8, byte: u8) {
    
  }

  /// Skip next instruction if Vx = Vy.
  /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
  fn se2(Vx: u8, Vy: u8) {

  }

  fn execute(opcode: u16) {
    match opcode {
      0x00E0 => Chip8::cls(),
      0x00EE => Chip8::ret(),
      _ => Chip8::execute_extended(opcode)
    }
  }

  fn execute_extended(opcode: u16) {

    let nnn: u16 = opcode & 0x0FFF;
    let x: u16 = (opcode >> 8) & 0x000F;
    let y: u16 = (opcode >> 4) & 0x000F;
    let kk: u16 = opcode & 0x00FF;
    let n: u16 = opcode & 0x000F;
            
    let opcode = 0x07;
    match opcode {
      0x0000 => {}, // Skip
      // 0x1000 => Chip8::jp(),
      // 0x2000 => Chip8::call(),
      // 0x3000 => Chip8::se(),
      // 0x4000 => Chip8::sne(),
      // 0x5000 => Chip8::se2(),
      _ => {}
    }
  }
}

pub enum OpCode {
  
}
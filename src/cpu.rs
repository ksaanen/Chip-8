use crate::{memory::Memory, registry::Registry, keyboard::Keyboard, display:: Display};

pub struct Cpu {
  memory: Memory,
  registry: Registry,
  keyboard: Keyboard,
  display: Display,
  can_wait_for_key_input: bool
}

impl Cpu {

  fn new(&mut self) {
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
  fn jp(addr) {

  }

  /// Call subroutine at nnn.
  /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
  fn call(addr) {

  }

  /// Skip next instruction if Vx = kk.
  /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
  fn se(Vx, byte) {

  }

  /// Skip next instruction if Vx != kk.
  /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
  fn sne(Vx, byte) {
    
  }

  /// Skip next instruction if Vx = Vy.
  /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
  fn se2(Vx, Vy) {

  }

  fn execute(opcode: u16) {
    match opcode {
      0x00E0 => Cpu::cls(),
      0x00EE => Cpu::ret(),
      _ => Cpu::execute_extended(opcode)
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
      0x0000 => {},
      0x1000 => Cpu::jp(),
      0x2000 => Cpu::call(),
      0x3000 => Cpu::se(),
      0x4000 => Cpu::sne(),
      0x5000 => Cpu::se2(),
      _ => {}
    }
  }
}

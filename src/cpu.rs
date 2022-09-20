use crate::{memory::Memory, registry::Registry, keyboard::Keyboard, display:: Display};

pub struct Cpu {
  memory: Memory,
  registry: Registry,
  keyboard: Keyboard,
  display: Display,
  can_wait_for_key_input: bool
}

impl Cpu {

  fn new(self) {
  }

  fn cls() {

  }

  fn ret() {
    
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
      0x07 => {

      },
      _ => {}
    }
  }
}

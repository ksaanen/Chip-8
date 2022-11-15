/// Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F). There is also a 16-bit register called I. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
pub struct Register {
  /// V register: has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F)
  pub v: [u8; 16],
  /// I register: is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
  pub i: u16,
  /// Stack Segment
  pub ss: [u16; 1],
  /// Delay Timer: the delay timer is active whenever the delay timer register (DT) is non-zero. This timer does nothing more than subtract 1 from the value of DT at a rate of 60Hz. When DT reaches 0, it deactivates.
  pub dt: u16,
  /// Sound Timer: the sound timer is active whenever the sound timer register (ST) is non-zero. This timer also decrements at a rate of 60Hz, however, as long as ST's value is greater than zero, the Chip-8 buzzer will sound. When ST reaches zero, the sound timer deactivates.
  pub st: u16,
  /// Program Counter: is used to store the currently executing address.
  pub pc: u16,
  /// Stack pointer: is used to point to the topmost level of the stack.
  pub sp: u16
}

/// Provides an implementation for the CHIP-8 registries.
impl Register {

  pub fn new() -> Register
  {
    return Register {
      v: [0x00; 16],
      i: 0x00,
      ss: [0x00],
      dt: 0x00,
      st: 0x00,
      pc: 0x00,
      sp: 0x00,
    }
  }

  fn push(&mut self, value: u16)
  {
    self.ss[self.sp as usize] = value;
    self.sp = self.sp + 1;
  }

  fn pop(&mut self) -> u16
  {
    self.sp = self.sp - 1;
    return self.ss[self.sp as usize];
  }
}
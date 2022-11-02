pub struct Register {
  V: [u8; 1],   // V register
  // I: u16,     // I register
  SS: [u16; 1],  // Stack Segment
  // DT: u16,    // Delay Timer
  // ST: u16,    // Sound Timer
  // PC: u16,    // Program Counter
  // SP: u16     // Stack Pointer
}

/// Provides an implementation for the CHIP-8 registries.
/// V-register - V
/// I-register - I
/// Stack or StackSegment - SS
/// DelayTimer - DT
/// SoundTimer - ST
/// ProgramCounter - PC
/// StackPointer - SP
impl Register {

  pub fn new(V: u8, S: u16) -> Register
  {
    return Register {
      V: [V],
      SS: [S]
    }
  }

  // fn push(&mut self, value: u16)
  // {
  //   self.SS[self.SP as usize] = value;
  //   self.SP = self.SP + 1;
  // }

  // fn pop(&mut self) -> u16
  // {
  //   self.SP = self.SP - 1;
  //   return self.SS[self.SP as usize];
  // }
}
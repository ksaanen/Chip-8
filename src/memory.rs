pub struct Memory {
  MEMORY_BUFFER: [u8; 4095],
}

impl Memory {

  pub fn new() -> Memory {
    return Memory {
      MEMORY_BUFFER: [0x00; 4095]
    }
  }

  fn read_from_memory(self, address: u8) -> u8 {
    // TODO: Take from index
    return self.MEMORY_BUFFER[address as usize];
  }

  fn write_to_memory(&mut self, address: u8, data: u8) {
    // TODO: Check if within memory range before writing
    self.MEMORY_BUFFER[address as usize] = data;
  }
}

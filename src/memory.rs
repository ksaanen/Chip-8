pub struct Memory {
  memory_buffer: [u8; 4095],
}

impl Memory {

  pub fn new() -> Memory {
    return Memory {
      memory_buffer: [0x00; 4095]
    }
  }

  pub fn read_from_memory(&self, address: u8) -> u8 {
    // TODO: Take from index
    return self.memory_buffer[address as usize];
  }

  pub fn write_to_memory(&mut self, address: u8, data: u8) {
    // TODO: Check if within memory range before writing
    self.memory_buffer[address as usize] = data;
  }
}

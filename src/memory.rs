pub struct Memory {
  ram: [u8; 4095],
}

impl Memory {

  pub fn new(&mut self) {
    self.ram = [0x00; 4095]; // Create empty ram instance
  }

  fn read_from_memory(self, address: u8) -> u8 {
    // TODO: Take from index
    return self.ram[address as usize];
  }

  fn write_to_memory(&mut self, address: u8, data: u8) {
    // TODO: Check if within memory range before writing
    self.ram[address as usize] = data;
  }
}

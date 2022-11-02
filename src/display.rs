pub struct Display {
  DISPLAY_WIDTH: i32,
  DISPLAY_HEIGHT: i32,
  DISPLAY_BUFFER: [u8; 2048]
}

impl Display {

  pub fn new() -> Display {
    return Display {
      DISPLAY_WIDTH: 64,
      DISPLAY_HEIGHT: 32,
      DISPLAY_BUFFER: [0x00; 2048] // Init display buffer array width * height
    }
  }

}
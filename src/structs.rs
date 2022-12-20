pub fn run() {
  struct Color {
    red: u8,
    green: u8,
    blue: u8,
  }

  let mut white = Color {
    red: 255,
    green: 255,
    blue: 255,
  };

  println!(
    "Here is the color from struct R: {} G: {} B: {}",
    white.red, white.green, white.blue
  );

  // Now lets add some functionality to the struct.
  // Unlike most other languages they are kept separate from the struct properties.
  // They are kept under an "impl(ementation)" block
  impl Color {
    fn print(&self) {
      println!(
        "Color Data: R: {} G: {} B: {}",
        self.red, self.green, self.blue
      )
    }
  }

  white.print();
}

// A tuple version would be as follows
// Struct Color(u8,u8,u8); indexing on this would be c.0, c.1, c.2

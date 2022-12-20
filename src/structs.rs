pub fn run() {
  let mut white = Color {
    red: 255,
    green: 255,
    blue: 255,
  };

  println!(
    "Here is the color from struct: \n R: {} G: {} B: {}",
    white.red, white.green, white.blue
  );
}
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// A tuple version would be as follows
// Struct Color(u8,u8,u8); indexing on this would be c.0, c.1, c.2

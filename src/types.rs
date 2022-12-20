pub fn run() {
  //Primitive types
  /*
      Intergers: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128
      Floats: f32,f64
      bool
      char
      Tuples
      Arrays
      Objects
  */
  //Explicit typing and casting
  let big_num: i64 = (std::i32::MAX as i64) + 1;
  println!("Bignum value: {}", big_num);
}

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
  println!("big_num value: {}", big_num);

  // To get the size in bytes of a type use the following (analgous to sizeof c++)
  println!(
    "Size of big_num in bytes: {}",
    std::mem::size_of_val(&big_num)
  );

  // All types in comment above match except tuples, arrays and objects.
  // Tuple declaration (Say this is first, last, favorite number)
  let person: (&str, &str, i32) = ("Haroon", "Syed", 11);
  println!(
    "Tuple data:\nFirst: {} Last: {} Favorite Number: {}",
    person.0, person.1, person.2
  );

  // Arrays and objects have their own examples

  // Casting string to int
  let str: String = "21".to_string();
  let num: i32 = str.parse().expect("Could not cast str to an i32");
  dbg!(str, num);
}

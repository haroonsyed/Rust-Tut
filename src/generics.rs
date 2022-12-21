use std::ops::Add;

// Like c/c++ we use T in the brackets. But they also need to share traits
// The trait guarantees it is valid to perform action between T
fn generic_add<T: Add<Output = T>>(x: T, y: T) -> T {
  return x + y;
}

pub fn run() {
  println!(
    "Sum of two floats using generic function: {}",
    generic_add(5.12, 6.85)
  );
  println!(
    "Sum of two int using generic function: {}",
    generic_add(5, 10)
  );
}

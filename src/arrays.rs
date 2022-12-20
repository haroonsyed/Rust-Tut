pub fn run() {
  //Fixed size and uniform type
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{:?}", numbers);
  println!("First element of array is: {}", numbers[0]);

  // To get a slice of an array
  let slice: &[i32] = &numbers[0..2];
  println!("{:?}", slice);
}

pub fn run() {
  let mut my_string = String::from("Hello");
  println!("Length of myString: {}", my_string.len());
  my_string.push_str(" World!");
  println!("New value of myString: {}", my_string);
  //There are a bunch of obvious functionality in the string class

  //One cool thing is that you can set a max capacity for strings
  let mut _capped = String::with_capacity(5);
}

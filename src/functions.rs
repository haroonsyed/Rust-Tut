pub fn run() {
  greeting("Haroon");
  println!("5+5 is {}", add(5, 5));

  let sum = add(5, 5);
  println!("Sum of 5,5 numbers from variable sum: {}", sum);

  //Closure (essentially storing an an anonymous function in a variable). Can also reference outer scope.
  let add_nums = |n1: i32, n2: i32| n1 + n2;
  println!("Sum of two nums 3 & 7 using a closure: {}", add_nums(3, 7));
}

//Takes in a pointer to a string variable (pass by ref)
fn greeting(name: &str) {
  println!("Hello {}", name);
}

//Primitives are copied in value
fn add(n1: i32, n2: i32) -> i32 {
  return n1 + n2;
}

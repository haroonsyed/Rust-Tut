pub fn run() {
  //Variables are IMMUTABLE by default
  //Rust is block scoped

  //Defining a variable:
  let name = "Haroon";
  println!("My name is {}", name);

  //Defining a mutable variable
  let mut color = "Red";
  println!("Favorite color {}", color);
  color = "Green";
  println!("Favorite color {}", color);

  //Defining constants
  const ID: i32 = 52;
  println!("Constant printed {}", ID);

  //Multivariable assignment
  let (_name2, _age2) = ("Syed", 20);
}

pub fn run() {
  println!("What's your name?");
  let mut name: String = String::new();
  std::io::stdin()
    .read_line(&mut name)
    .expect("Please pass in a string!");

  println!("Nice to meet you, {}", name);
}

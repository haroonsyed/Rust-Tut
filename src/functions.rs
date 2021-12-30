pub fn run() {
    greeting("Haroon");
    println!("5+5 is {}", add(5,5))
}

//Takes in a pointer to a string variable (pass by ref)
fn greeting(name: &str) {
    println!("Hello {}",name);
}

//Primitives are copied in value
fn add(n1: i32, n2: i32)->i32 {
    return n1+n2;
}
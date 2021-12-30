pub fn run() {
    //Pretty much how c works
    let age = 18;
    let registered = true;
    if(age>=18) {
        println!("You are now an adult!");
    }
    else {
        println!("You are not an adult.");
    }
    if(age >= 18 && registered) {
        println!("You can vote!");
    }
}
use std::ops::Add;

pub fn run() {
    let mut myString = String::from("Hello");
    println!("Length of myString: {}",myString.len());
    myString.push_str(" World!");
    println!("New value of myString: {}",myString);
    //There are a bunch of obvious functionality in the string class

    //One cool thing is that you can set a max capacity for strings
    let mut capped = String::with_capacity(5);
}
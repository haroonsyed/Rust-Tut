//Here is how to define a function
pub fn run() {
    //Basic print
    println!("Hello from print module!");

    //Positional Argument
    println!("My name is {1} my age is {0}",20,"Haroon");

    //Named arguments
    println!("My name is {name} my age is {age}",age=20,name="Haroon");

    //Placeholder traits let you convert a number to different bases on the fly

    //What's especially useful is using placeholder for debugging
    println!("{:?}",(12,true,"hello"));

}
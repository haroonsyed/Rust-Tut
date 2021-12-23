pub fn run() {
    //Dynamic size and uniform
    let mut numbers:Vec<i32> = vec![1];
    println!("{:?}",numbers);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    println!("First element of array is: {}",numbers[0]);
}
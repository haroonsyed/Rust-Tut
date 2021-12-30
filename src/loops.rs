pub fn run() {
    let mut count = 0;

    //Infinite loop
    loop {
        count+=1;
        println!("Count in infinite while : {}", count);
        if(count == 10) {
            break;
        }
    }
    count=0;

    while count<10 {
        count+=1;
        println!("Count in conditional while: {}", count);
    }

    //Range based
    for x in 1..11 {
        println!("Count in range based loop: {}", x);
    }



}
// Control the flow
pub fn let_it_flow(x: i32) {
    // if statement
    if x > 10 {
        println!("x is greater than 10");
    } else {
        println!("x is less than or equal to 10");
    }

    if x != 0 {
        println!("x is not zero");
    }

    if x % 2 == 0 {
        println!("x is even");
    } else {
        println!("x is odd");
    }

    // loop statement
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            println!("Counter reached 10");
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    // while statement
    let mut count_down = 10;
    while count_down != 0 {
        println!("{}!", count_down);
        count_down -= 1;
    }
    println!("Goooooo!!!");

    // for statement
    let weeks = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    for weekday in weeks {
        println!("{}", weekday);
    }

    // Countdown using for statement
    for number in (1..10).rev() {
        println!("{}!", number);
    }
}
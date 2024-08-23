use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Try constants
    println!("** Constants **");
    constant_example();

    // Try variables types
    println!("** Variables types **");
    var_types();

    // Try compound types
    println!("** Compound types **");
    var_compound_types();

    // Try mutability
    println!("** Mutability **");
    var_mutability();

    // Try shadowing
    println!("** Shadowing **");
    var_shadowing();

    // Try println! macro
    println!("** println! macro **");
    println_macro();

    // Lucky number function
    println!("** Lucky number function **");
    let lucky_number = my_lucky_number();
    println!("My lucky number is: {}", lucky_number);

    // Function with arguments
    println!("** Function with arguments **");
    another_function(5, 'c', "this is a test");

    // Function with return value
    println!("** Function with return value **");
    let result = function_return_value(5);
    println!("The result is: {}", result);

    // Control the flow
    println!("** Control the flow **");
    let_it_flow(23);

    // Guess a number game
    println!("** Guess a number **");
    guess_a_number();
}

// Experiment with constants
fn constant_example() {
    const MAX_POINTS: u32 = 100000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}

// Experiment with variables types
fn var_types() {
    let example: i32 = 5; // i32
    println!("Example 1: {}", example);
    let example: f64 = 5.0; // f64
    println!("Example 2: {}", example);
    let example: f32 = 23.123; // f32
    println!("Example 3: {}", example);
    let example: i32 = 5_000; // i32
    println!("Example 4: {}", example);
    let example: f64 = 0.000_005; // f64
    println!("Example 5: {}", example);
    let example: u8 = 5; // u8 ---> if you use u8 = 256 it will not compile
    println!("Example 6: {}", example);
    let example: bool = true; // bool
    println!("Example 7: {}", example);
    let example: char = 'z'; // char
    println!("Example 8: {}", example);
    let example: &str = "Hello, world!"; // string
    println!("Example 9: {}", example);
}

// Experiment with compound types
fn var_compound_types() {
    // Tuples
    let tup: (i32, f64, char) = (500, 123.23, 'z');
    println!("The value of tup is: ({}, {}, {})", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("The value of x, y, z is: ({}, {}, {})", x, y, z);

    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("The value of x, y, z is: ({}, {}, {})", x, y, z);

    // Arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array is: [{}, {}, {}, {}, {}]", array[0], array[1], array[2], array[3], array[4]);
    array.iter().for_each(|x| println!("{} ", x));

    let weeks = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    weeks.iter().for_each(|x| println!("{}", x));
}

// Experiment with shadowing
fn var_shadowing() {
    let x: i32 = 5;
    let x: i32 = x + 1; // Shadowing using let

    {
        // Shadowing in inner scope
        let x: i32 = x * 2;
        println!("The value of x in inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);
}

// Experiment with mutability
fn var_mutability() {
    let mut x = 5; // mutable variable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let appels = 5; // immutable variable
    let mut bananas = 5; // mutable variable
    println!("appels: {}", appels);
    println!("bananas: {}", bananas);

    // appels = 6; // this would not compile
    bananas = 6;
    println!("appels: {}", appels);
    println!("bananas: {}", bananas);
}

// Experiment with println! macro
fn println_macro() {
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);
    println!("x = {x} and y = {y}");
    println!("x = {x} and y = {y} and the sum is {sum}", x = x, y = y, sum = x + y);
}

// Experiment with user input
fn guess_a_number() {
    loop {
        // Generate a random number between 1 and 100
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {}", secret_number);

        let mut guess = String::new();

        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingoooo!!");
                break;
            }
        }
    }
}

// Function with arguments
fn another_function(x: i32, unit_label: char, unit_type: &str) {
    println!("The value of x is: {}", x);
    println!("The unit label is: {}", unit_label);
    println!("The unit type is: {}", unit_type);
}

// Function with return value
fn function_return_value(x: i32) -> i32 {
    let y = {
        let x = 3;
        x + 1
    };

    x + y
}

// The simplest function
fn my_lucky_number() -> i32 {
    23
}

// Control the flow
fn let_it_flow(x: i32) {
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
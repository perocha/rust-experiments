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

    // Try interesting stuff with vars
    println!("** Interesting stuff with vars **");
    var_interesting();

    // Try mutability
    println!("** Mutability **");
    var_mutability();

    // Try a bit more of mutability
    println!("** Mutability test **");
    mutability_test();

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

    // Heap memory test
    println!("** Heap memory test **");
    heap_memory_test();

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

// Interesting stuff with vars
fn var_interesting() {
    // Infer type i32
    let mut infer_type_i32 = 1;
    println!("The value of infer_type_i32 is: {}", infer_type_i32);
    infer_type_i32 = 2_147_483_647;
    println!("The value of infer_type_i32 is: {}", infer_type_i32);

    // Infer type i64
    let mut infer_type_i64 = 1;
    println!("The value of infer_type_i64 is: {}", infer_type_i64);
    infer_type_i64 = 2_147_483_648i64; // notice the i64 suffix
    println!("The value of infer_type_i64 is: {}", infer_type_i64);
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

// Heap memory test
fn heap_memory_test() {
    let x = 65315;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1 = {}", s1);   ---> this will not compile, s1 was moved to s2
    println!("s2 = {}", s2);

    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // Take ownership test
    take_ownership(s3);
    println!("s2 = {}", s2);
    // println!("s3 = {}", s3);   ---> this will not compile, s3 was moved to ownership_test

    // Give ownership test
    let s3 = give_ownership();
    println!("s3 = {}", s3);

    // Take and give back ownership test
    let s4 = String::from("hello");
    let s5 = take_and_give_back_ownership(s4);
    // println!("s4 = {}", s4);   ---> this will not compile, s4 was moved to s5
    println!("s5 = {}", s5);

    // Use s5 string without taking ownership, by passing a reference (a read-only reference)
    let length = calculate_length(&s5);
    println!("The length of '{}' is: {}", s5, length);

    // Now pass a mutable reference, to modify the string
    let mut s6 = String::from("hello");
    modify_string(&mut s6);
    println!("s6 = {}", s6);
}

// Take ownership test
fn take_ownership(input_string: String) {
    println!("{}", input_string);
}

// Give ownership test
fn give_ownership() -> String {
    let s = String::from("hello");
    s
}

// Takes and gives back ownership
fn take_and_give_back_ownership(input_string: String) -> String {
    input_string
}

// Calculate the length of a string (without taking ownership of the input string, like it happens with take_ownership example)
fn calculate_length(input_string: &String) -> usize {
    input_string.len()
}

// Modify the content of a string using a mutable reference
fn modify_string(input_string: &mut String) {
    input_string.push_str(" world!");
}

// Mutability test
fn mutability_test() {
    let mut s = String::from("hello");

    // We can reference the same variable multiple times, except when we have a mutable reference
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");

    // Now we have a mutable reference, so we can't have any other reference to the same variable
    let r3 = &mut s; // no problem
    println!("{r3}");

    // println!("r1 = {r1}"); ---> this will not compile, s is borrowed as mutable
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

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Try constants
    println!("** Constants **");
    constant_example();

    // Try mutability
    println!("** Mutability **");
    var_mutability();

    // Try shadowing
    println!("** Shadowing **");
    var_shadowing();

    // Try println! macro
    println!("** println! macro **");
    println_macro();

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
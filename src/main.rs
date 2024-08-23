use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Try mutability
    mutability();

    // Try println! macro
    println_macro();

    // Try user input
    get_userinput();
}

// Experiment with mutability
fn mutability() {
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
fn get_userinput() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Bingoooo!!"),
    }
}
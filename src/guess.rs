use std::io;
use rand::Rng;
use std::cmp::Ordering;

// Experiment with user input
pub fn guess_a_number() {
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
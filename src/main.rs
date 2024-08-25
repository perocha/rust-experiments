mod vars;
mod compoundtypes;
mod functions;
mod guess;
mod mutability;
mod memory;
mod flow;

fn main() {
    // Try constants
    println!("** Constants **");
    vars::constant_example();

    // Try variables types
    println!("** Variables types **");
    vars::var_types();

    // Try interesting stuff with vars
    println!("** Interesting stuff with vars **");
    vars::var_interesting();

    // Try string slices
    println!("** String slices **");
    vars::string_slice();

    // Try first word function
    println!("** First word function **");
    let input_string = String::from("This is a big string that I want to test");
    let first_word = vars::first_word(&input_string);
    println!("The string is: {}", input_string);
    println!("The first word is: {}", first_word);

    // Try compound types
    println!("** Compound types **");
    compoundtypes::var_compound_types();

    // Try mutability
    println!("** Mutability **");
    mutability::var_mutability();

    // Try a bit more of mutability
    println!("** Mutability test **");
    mutability::mutability_test();

    // Try shadowing
    println!("** Shadowing **");
    vars::var_shadowing();

    // Try println! macro
    println!("** println! macro **");
    println_macro();

    // Lucky number function
    println!("** Lucky number function **");
    let lucky_number = functions::my_lucky_number();
    println!("My lucky number is: {}", lucky_number);

    // Function with arguments
    println!("** Function with arguments **");
    functions::another_function(5, 'c', "this is a test");

    // Function with return value
    println!("** Function with return value **");
    let result = functions::function_return_value(5);
    println!("The result is: {}", result);

    // Control the flow
    println!("** Control the flow **");
    flow::let_it_flow(23);

    // Heap memory test
    println!("** Heap memory test **");
    memory::heap_memory_test();

    // Guess a number game
    println!("** Guess a number **");
    guess::guess_a_number();
}

// Experiment with println! macro
fn println_macro() {
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);
    println!("x = {x} and y = {y}");
    println!("x = {x} and y = {y} and the sum is {sum}", x = x, y = y, sum = x + y);
}
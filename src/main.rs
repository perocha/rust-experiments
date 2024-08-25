mod vars;
mod compoundtypes;
mod functions;
mod guess;
mod mutability;
mod memory;
mod flow;
mod utils;

mod registry;
use registry::user::{User, UserProfile};
use registry::patient::Patient;
use registry::registry::Registry;

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

    // Helper function
    println!("** Helper function **");
    utils::helper::helper_function();

    // Registry
    println!("** The registry **");
    patient_registry();
}

// Experiment with println! macro
fn println_macro() {
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);
    println!("x = {x} and y = {y}");
    println!("x = {x} and y = {y} and the sum is {sum}", x = x, y = y, sum = x + y);
}

// Experiment with Patient registry
fn patient_registry() {
    // Create a new registry
    let mut registry = Registry::new();

    // Create new users
    let guardian = User {
        id: 1,
        name: String::from("John Doe"),
        profile: UserProfile::LegalGuardian,
    };
    let admin = User {
        id: 2,
        name: String::from("Peter Rock"),
        profile: UserProfile::Admin,
    };
    let doctor = User {
        id: 3,
        name: String::from("Dr. Smith"),
        profile: UserProfile::Doctor,
    };

    // Add users to the registry
    registry.add_user(guardian);
    registry.add_user(admin);
    registry.add_user(doctor);

    // Create a patient
    let patient = Patient {
        id: 1,
        name: String::from("Jane Doe"),
        age: 10,
        guardian_id: 1,
    };

    // Add patient to the registry
    registry.add_patient(patient);

    // Print the first user
    let first_user = &registry.users[0];
    println!("The first user is {}", first_user.name);
}
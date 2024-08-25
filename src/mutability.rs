// Experiment with mutability
pub fn var_mutability() {
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

// Mutability test
pub fn mutability_test() {
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
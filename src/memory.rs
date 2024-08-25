// Heap memory test
pub fn heap_memory_test() {
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
pub fn take_ownership(input_string: String) {
    println!("{}", input_string);
}

// Give ownership test
pub fn give_ownership() -> String {
    let s = String::from("hello");
    s
}

// Takes and gives back ownership
pub fn take_and_give_back_ownership(input_string: String) -> String {
    input_string
}

// Calculate the length of a string (without taking ownership of the input string, like it happens with take_ownership example)
pub fn calculate_length(input_string: &String) -> usize {
    input_string.len()
}

// Modify the content of a string using a mutable reference
pub fn modify_string(input_string: &mut String) {
    input_string.push_str(" world!");
}
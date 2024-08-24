// Experiment with constants
pub fn constant_example() {
    const MAX_POINTS: u32 = 100000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}

// Experiment with variables types
pub fn var_types() {
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
pub fn var_interesting() {
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

// Experimenting with string slices
pub fn string_slice() {
    let s = String::from("hello world");

    // Slicing a string, by using a range [starting_index..ending_index]
    let hello = &s[0..5];
    let another_hello = &s[..5]; // same as above
    let world = &s[6..11];
    let hello_world = &s[..]; // same as &s[0..11]
    println!("hello: {hello}");
    println!("another_hello: {another_hello}");
    println!("world: {world}");
    println!("hello_world: {hello_world}");
}

// Return the first word of a string
pub fn first_word(input_string: &str) -> &str {
    let bytes = input_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input_string[..i];
        }
    }

    &input_string[..]
}
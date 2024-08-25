// Function with arguments
pub fn another_function(x: i32, unit_label: char, unit_type: &str) {
    println!("The value of x is: {}", x);
    println!("The unit label is: {}", unit_label);
    println!("The unit type is: {}", unit_type);
}

// Function with return value
pub fn function_return_value(x: i32) -> i32 {
    let y = {
        let x = 3;
        x + 1
    };

    x + y
}

// The simplest function
pub fn my_lucky_number() -> i32 {
    23
}
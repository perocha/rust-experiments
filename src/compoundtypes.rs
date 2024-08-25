// Experiment with compound types
pub fn var_compound_types() {
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

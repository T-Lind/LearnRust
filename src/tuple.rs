pub(crate) fn learn_tuple() {
    // Different data types for a and b here
    let tuple0 = (2, 3.4);
    let n0 = tuple0.0; // Cannot use brackets
    let n1 = tuple0.1;
    println!("Tuple: ({n0}, {n1})");

    // Import a function - minimum from standard library
    use std::cmp::min;
    // Casting example as below
    let least = min(tuple0.0, tuple0.1 as i32);
    println!("Least in tuple: {least}")
}
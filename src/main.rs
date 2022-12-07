fn dice_roll() -> i8 {
    return 3;
}



fn main() {
    let evaluate = {
        let a = 3;
        let b = 2;
        a + b * a
    };

    let x0 = 3;
    // 32 bits explicitly
    let x1: i32 = 3;
    println!("Numbers: {x0} {x1}");
    println!("let evaluation: {evaluate}");

    // Different data types for a and b here
    let tuple0 = (2, 3.4);
    let n0 = tuple0.0; // Cannot use brackets
    let n1 = tuple0.1;
    println!("Tuple: ({n0}, {n1})");

    use std::cmp::min;
    let least = min(tuple0.0 as f64, tuple0.1);
    println!("Least in tuple: {least}")
}
